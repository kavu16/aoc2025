use std::iter::repeat_n;

/*
 * 11 - 22 -> 2
 * 13 - 22 -> 2 - 1 + 1 = 2
 * 13 -21 ->
 */
pub fn solve1(data: &str) -> usize {
    let res = data
        .split(',')
        .flat_map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start = start.trim().parse::<usize>().unwrap();
            let end = end.trim().parse::<usize>().unwrap();
            start..=end
        })
        .filter(|n| {
            let n_string = n.to_string();
            n_string[..n_string.len() / 2] == n_string[n_string.len() / 2..]
        })
        .sum();
    println!("Day2 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> usize {
    let res = data
        .split(',')
        .flat_map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start = start.trim().parse::<usize>().unwrap();
            let end = end.trim().parse::<usize>().unwrap();
            start..=end
        })
        .filter(|n| {
            let n_string = n.to_string();
            (1..n_string.len() / 2 + 1).any(|i| {
                n_string.len() % i == 0
                    && n_string
                        == repeat_n(n_string[..i].to_string(), n_string.len() / i)
                            .collect::<String>()
            })
        })
        .sum();
    println!("Day2 Part 1 = {res}");
    res
}

#[test]
fn test_example() {
    let data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(solve1(data), 1227775554);
    assert_eq!(solve2(data), 4174379265);
}
