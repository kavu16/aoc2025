use itertools::Itertools;
use std::collections::BTreeSet;

pub fn solve1(data: &str) -> u64 {
    let res = data.lines().map(|bank| find_max_battery(bank, 2)).sum();

    println!("Day 3 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> u64 {
    let res = data.lines().map(|bank| find_max_battery(bank, 12)).sum();

    println!("Day 3 Part 2 = {res}");
    res
}

fn find_max_battery(bank: &str, size: usize) -> u64 {
    let bank = bank.as_bytes().iter().enumerate().collect::<Vec<_>>();
    let mut cache = BTreeSet::new();
    find_batteries(&bank, size, &mut cache);
    cache
        .into_iter()
        .map(|(_idx, b)| b as char)
        .join("")
        .parse::<u64>()
        .unwrap()
}

fn find_batteries(bank: &[(usize, &u8)], battery_num: usize, cache: &mut BTreeSet<(usize, u8)>) {
    if battery_num == 0 {
        return;
    }
    let size = bank.len();
    let (max_idx, (max_og_idx, max)) = bank.iter().enumerate().fold(
        (0usize, (0usize, 0u8)),
        |(max_idx, (max_og_idx, max)), (idx, (og_idx, digit))| {
            if **digit > max {
                (idx, (*og_idx, **digit))
            } else {
                (max_idx, (max_og_idx, max))
            }
        },
    );

    cache.insert((max_og_idx, max));
    let remain = size - max_idx - 1;
    if remain >= battery_num - 1 {
        find_batteries(&bank[max_idx + 1..], battery_num - 1, cache);
    } else {
        find_batteries(&bank[..max_idx], battery_num - 1 - remain, cache);
        find_batteries(&bank[max_idx + 1..], remain, cache);
    }
}

#[test]
fn test_example() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    assert_eq!(solve1(input), 357);
    assert_eq!(solve2(input), 3121910778619);
}

#[test]
fn text_max_battery() {
    let bank = "234234234234278";
    assert_eq!(find_max_battery(bank, 12), 434234234278);
}
