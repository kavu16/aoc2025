use std::collections::BTreeSet;

pub fn solve1(data: &str) -> usize {
    let (ranges, ingredients) = data.split_once("\n\n").unwrap();

    let mut range_set = ranges.lines().fold(BTreeSet::new(), |mut range_set, line| {
        let (start, end): (u64, u64) = line
            .split_once('-')
            .and_then(|(s, e)| s.parse().ok().zip(e.parse().ok()))
            .expect("Invalid range");

        range_set.insert((start, end));
        range_set
    });

    let mut range_vec: Vec<(u64, u64)> = vec![];
    while let Some((start, mut end)) = range_set.pop_first() {
        if let Some(rear) = range_vec.last()
            && rear.1 >= end
        {
            continue;
        }

        while let Some(&(front, _back)) = range_set.first()
            && front <= end
        {
            end = end.max(range_set.pop_first().unwrap().1);
        }
        range_vec.push((start, end));
    }

    let range_vec = range_vec
        .into_iter()
        .map(|(start, end)| start..=end)
        .collect::<Vec<_>>();

    let res = ingredients
        .lines()
        .flat_map(|ingredient| ingredient.parse::<u64>())
        .filter(|&ingredient| range_vec.iter().any(|range| range.contains(&ingredient)))
        .count();
    println!("Day 5 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> u64 {
    let (ranges, _ingredients) = data.split_once("\n\n").unwrap();

    let mut range_set = ranges.lines().fold(BTreeSet::new(), |mut range_set, line| {
        let (start, end): (u64, u64) = line
            .split_once('-')
            .and_then(|(s, e)| s.parse().ok().zip(e.parse().ok()))
            .expect("Invalid range");

        range_set.insert((start, end));
        range_set
    });

    let mut range_vec: Vec<(u64, u64)> = vec![];
    while let Some((start, mut end)) = range_set.pop_first() {
        if let Some(rear) = range_vec.last()
            && rear.1 >= end
        {
            continue;
        }

        for &(front, back) in &range_set {
            if front <= end {
                end = end.max(back);
            }
        }
        range_vec.push((start, end));
    }

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
