use std::{collections::HashSet, iter::successors};

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn find_rolls(data: &str) -> HashSet<(i32, i32)> {
    data.lines()
        .enumerate()
        .fold(HashSet::new(), |mut map, (r, row)| {
            map.extend(
                row.char_indices()
                    .filter(|(_c, roll)| *roll == '@')
                    .map(|(c, _roll)| (r as i32, c as i32)),
            );
            map
        })
}

fn accessible((r, c): &(i32, i32), roll_map: &HashSet<(i32, i32)>) -> bool {
    DIRECTIONS
        .iter()
        .filter(|(dr, dc)| roll_map.contains(&(r + dr, c + dc)))
        .count()
        < 4
}

pub fn solve1(data: &str) -> usize {
    let roll_map = find_rolls(data);
    let res = roll_map
        .iter()
        .filter(|coord| accessible(coord, &roll_map))
        .count();

    println!("Day 4 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> usize {
    let res = successors(Some((0, find_rolls(data))), |(_, roll_map)| {
        let (removed, remain): (HashSet<_>, HashSet<_>) = roll_map
            .iter()
            .partition(|coord| accessible(coord, &roll_map));
        if removed.len() == 0 {
            None
        } else {
            Some((removed.len(), remain))
        }
    })
    .map(|(removed, _)| removed)
    .sum();

    println!("Day 4 Part 2 = {res}");
    res
}

#[test]
fn test_example() {
    let data = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    assert_eq!(solve1(data), 13);
    assert_eq!(solve2(data), 43);
}
