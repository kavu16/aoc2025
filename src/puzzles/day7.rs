use std::collections::HashMap;

pub fn solve1(data: &str) -> usize {
    let mut grid = data
        .lines()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut total_splits = 0;
    for r in 1..grid.len() {
        for c in 0..grid[r].len() {
            match grid[r - 1][c] {
                'S' => {
                    grid[r][c] = '|';
                }
                '|' if grid[r][c] == '^' => {
                    if c > 0 {
                        grid[r][c - 1] = '|';
                    }
                    if c < grid[r].len() - 1 {
                        grid[r][c + 1] = '|';
                    }
                    total_splits += 1;
                }
                '|' => {
                    grid[r][c] = '|';
                }
                _ => continue,
            }
        }
    }

    let res = total_splits;
    println!("Day 7 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> usize {
    let grid = data
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    'S' | '^' => 1,
                    _ => 0,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // let mut cache = HashMap::new();
    // let start = grid[0].iter().position(|c| *c == 'S').unwrap();
    // let paths = tachyon_paths(1, start, &grid, &mut cache);
    let paths = tachyon_paths_iter(grid);
    println!("Day 7 Part 2 = {paths}");
    paths
}

fn tachyon_paths_iter(grid: Vec<Vec<usize>>) -> usize {
    grid.into_iter()
        .reduce(|mut prev, curr| {
            for (i, &c) in curr.iter().enumerate() {
                if c == 1 {
                    if i > 0 {
                        prev[i - 1] += prev[i];
                    }
                    if i < curr.len() - 1 {
                        prev[i + 1] += prev[i];
                    }
                    prev[i] = 0;
                }
            }
            prev
        })
        .unwrap_or_default()
        .into_iter()
        .sum()
}

fn _tachyon_paths(
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&paths) = cache.get(&(row, col)) {
        return paths;
    }

    if row >= grid.len() {
        return 1;
    }

    let paths = match grid[row][col] {
        '.' => _tachyon_paths(row + 1, col, grid, cache),
        '^' => {
            let left = if col > 0 {
                _tachyon_paths(row + 1, col - 1, grid, cache)
            } else {
                0
            };
            let right = if col < grid[row].len() - 1 {
                _tachyon_paths(row + 1, col + 1, grid, cache)
            } else {
                0
            };
            left + right
        }
        _ => 0,
    };

    cache.insert((row, col), paths);
    paths
}

#[test]
fn test_example() {
    let data = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    assert_eq!(solve1(data), 21);
    assert_eq!(solve2(data), 40);
}
