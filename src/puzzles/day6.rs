pub fn solve1(data: &str) -> usize {
    let ops = data
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>();
    let res = data
        .lines()
        .take_while(|line| {
            line.trim()
                .starts_with(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
        })
        .map(|line| {
            line.trim()
                .split_whitespace()
                .flat_map(|n| n.parse::<usize>())
                .collect::<Vec<_>>()
        })
        .reduce(|agg, row| {
            row.iter()
                .zip(agg.iter())
                .enumerate()
                .map(|(idx, (&a, &b))| match ops[idx] {
                    "+" => b + a,
                    "*" => b * a,
                    _ => b,
                })
                .collect()
        })
        .unwrap_or_default()
        .into_iter()
        .sum();
    println!("Day 6 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> usize {
    let max_width = data.lines().map(|line| line.len()).max().unwrap();
    let chars = data
        .lines()
        .map(|line| {
            let mut char_vec = line.chars().collect::<Vec<_>>();
            if char_vec.len() < max_width {
                char_vec.extend(vec![' '; max_width - char_vec.len()]);
            }
            char_vec
        })
        .collect::<Vec<_>>();

    let transpose = transpose(chars);
    let (total, curr, _) = transpose
        .iter()
        .map(|row| row.iter().collect::<String>())
        .fold((0, 0, '+'), |(total, mut curr, mut op), mut num| {
            if num.ends_with(['+', '*']) {
                op = num.pop().unwrap();
                curr = match op {
                    '+' => 0,
                    '*' => 1,
                    _ => 0,
                };
            }
            if let Ok(num) = num.as_str().trim().parse::<usize>() {
                match op {
                    '+' => (total, curr + num, op),
                    '*' => (total, curr * num, op),
                    _ => (total, curr, op),
                }
            } else {
                (total + curr, 0, op)
            }
        });
    let res = total + curr;
    println!("Day 6 Part 2 = {res}");
    res
}

fn transpose<T: Default + Clone + Copy>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut output = vec![vec![T::default(); input.len()]; input[0].len()];
    for r in 0..input.len() {
        for c in 0..input[0].len() {
            output[c][r] = input[r][c];
        }
    }
    output
}

#[test]
fn test_transpose() {
    let input = vec![
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'],
    ];
    let expected = vec![
        vec!['a', 'd', 'g'],
        vec!['b', 'e', 'h'],
        vec!['c', 'f', 'i'],
    ];
    let output = transpose(input);
    assert_eq!(output, expected);
    for row in output {
        println!("{}", row.iter().collect::<String>());
    }
}

#[test]
fn test_example() {
    let data = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
    assert_eq!(solve1(data), 4277556);
    assert_eq!(solve2(data), 3263827)
}

#[test]
fn test_parse() {
    let example = "39   ";
    let num = example.parse::<usize>();
    assert!(num.is_err());
}
