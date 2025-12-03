enum Turn {
    Left(i32),
    Right(i32),
}

impl From<&str> for Turn {
    fn from(s: &str) -> Self {
        match s.chars().collect::<Vec<char>>().as_slice() {
            ['L', val @ ..] => Turn::Left(val.iter().collect::<String>().parse::<i32>().unwrap()),
            ['R', val @ ..] => Turn::Right(val.iter().collect::<String>().parse::<i32>().unwrap()),
            _ => panic!("Invalid turn"),
        }
    }
}

pub fn solve1(data: &str) -> i32 {
    let (_, zeros) = data
        .lines()
        .fold((50, 0), |(dial_pos, zero_count), turn| match turn.into() {
            Turn::Left(clicks) => {
                let new_pos = (dial_pos - clicks).rem_euclid(100i32);
                (new_pos, zero_count + if new_pos == 0 { 1 } else { 0 })
            }
            Turn::Right(clicks) => {
                let new_pos = (dial_pos + clicks) % 100;
                (new_pos, zero_count + if new_pos == 0 { 1 } else { 0 })
            }
        });

    println!("Day 1 Part 1 = {zeros}");
    zeros
}

pub fn solve2(data: &str) -> i32 {
    let (_, zeros) = data
        .lines()
        .fold((50, 0), |(dial_pos, zero_count), turn| match turn.into() {
            Turn::Left(clicks) => {
                let new_pos = (dial_pos - clicks).rem_euclid(100i32);
                let new_zeroes = if dial_pos == 0 {
                    clicks / 100
                } else if clicks >= dial_pos {
                    1 + (clicks - dial_pos) / 100
                } else {
                    0
                };
                // println!("started at {dial_pos}, Left {clicks} passed zero {new_zeroes} times");
                (new_pos, zero_count + new_zeroes)
            }
            Turn::Right(clicks) => {
                let new_pos = (dial_pos + clicks) % 100;
                (new_pos, zero_count + (dial_pos + clicks) / 100)
            }
        });

    println!("Day 1 Part 2 = {zeros}");
    zeros
}

#[test]
fn test_example() {
    let data = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    assert_eq!(solve1(data), 3);
    assert_eq!(solve2(data), 6);
}
