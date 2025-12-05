use std::collections::BTreeSet;

pub fn solve1(data: &str) -> usize {
    let (ranges, ingredients) = data.split_once("\n\n").unwrap();

    let range_set: BTreeSet<(u64, u64)> = ranges
        .lines()
        .map(|line| {
            let (start, end): (u64, u64) = line
                .split_once('-')
                .and_then(|(s, e)| s.parse().ok().zip(e.parse().ok()))
                .expect("Invalid range");

            (start, end)
        })
        .collect();

    let range_vec: Vec<(u64, u64)> = range_set.into_iter().fold(vec![], |mut rv, (start, end)| {
        if rv.is_empty() || rv.last().unwrap().1 < start {
            rv.push((start, end));
        } else {
            let (front, back) = rv.pop().unwrap();
            rv.push((front.min(start), end.max(back)));
        }
        rv
    });

    let res = ingredients
        .lines()
        .flat_map(|ingredient| ingredient.parse())
        .filter(|ingredient| {
            range_vec
                .iter()
                .map(|&(start, end)| start..=end)
                .any(|range| range.contains(ingredient))
        })
        .count();
    println!("Day 5 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> u64 {
    let (ranges, _ingredients) = data.split_once("\n\n").unwrap();

    let range_set: BTreeSet<(u64, u64)> = ranges
        .lines()
        .map(|line| {
            let (start, end): (u64, u64) = line
                .split_once('-')
                .and_then(|(s, e)| s.parse().ok().zip(e.parse().ok()))
                .expect("Invalid range");

            (start, end)
        })
        .collect();

    let range_vec: Vec<(u64, u64)> = range_set.into_iter().fold(vec![], |mut rv, (start, end)| {
        if rv.is_empty() || rv.last().unwrap().1 < start {
            rv.push((start, end));
        } else {
            let (front, back) = rv.pop().unwrap();
            rv.push((front.min(start), end.max(back)));
        }
        rv
    });

    let res = range_vec
        .into_iter()
        .map(|(start, end)| end - start + 1)
        .sum();
    println!("Day 5 Part 1 = {res}");
    res
}

#[test]
fn test_example() {
    let data = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    assert_eq!(solve1(data), 3);
    assert_eq!(solve2(data), 14);
}
