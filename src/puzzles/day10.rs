use std::collections::{HashSet, VecDeque};

use z3::{Optimize, ast::Int};

pub fn solve1(data: &str) -> usize {
    let mut indicators: Vec<String> = Vec::new();
    let mut buttons: Vec<Vec<String>> = Vec::new();
    let mut joltages: Vec<String> = Vec::new();
    for line in data.lines() {
        let mut machine_buttons = Vec::new();
        for group in line.split_whitespace() {
            match group.chars().collect::<Vec<_>>().as_slice() {
                ['[', i @ .., ']'] => indicators.push(i.into_iter().collect()),
                ['(', b @ .., ')'] => machine_buttons.push(b.into_iter().collect()),
                ['{', j @ .., '}'] => joltages.push(j.into_iter().collect()),
                _ => {}
            }
        }
        buttons.push(machine_buttons);
    }
    let res = indicators
        .into_iter()
        .zip(buttons)
        .map(|(target, machine_buttons)| {
            let target: usize = target
                .char_indices()
                .map(|d| match d {
                    (_, '.') => 0,
                    (idx, '#') => 1 << idx,
                    _ => unreachable!(),
                })
                .sum();

            let machine_buttons: Vec<Vec<usize>> = machine_buttons
                .into_iter()
                .map(|button| button.split(',').flat_map(|d| d.parse::<usize>()).collect())
                .collect();
            let mut seen = HashSet::from([0]);
            let mut queue = VecDeque::from([(0, 0)]);
            while let Some((curr, presses)) = queue.pop_front() {
                if curr == target {
                    return presses;
                }
                for button in &machine_buttons {
                    let mut next = curr;
                    for idx in button {
                        next ^= 1 << idx;
                    }
                    if !seen.contains(&next) {
                        seen.insert(next);
                        queue.push_back((next, presses + 1));
                    }
                }
            }
            0
        })
        .sum();
    println!("Day 10 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> usize {
    let mut indicators: Vec<String> = Vec::new();
    let mut buttons: Vec<Vec<String>> = Vec::new();
    let mut joltages: Vec<String> = Vec::new();
    for line in data.lines() {
        let mut machine_buttons = Vec::new();
        for group in line.split_whitespace() {
            match group.chars().collect::<Vec<_>>().as_slice() {
                ['[', i @ .., ']'] => indicators.push(i.into_iter().collect()),
                ['(', b @ .., ')'] => machine_buttons.push(b.into_iter().collect()),
                ['{', j @ .., '}'] => joltages.push(j.into_iter().collect()),
                _ => {}
            }
        }
        buttons.push(machine_buttons);
    }
    let res = joltages
        .into_iter()
        .zip(buttons.into_iter())
        .map(|(target, machine_buttons)| {
            let target: Vec<usize> = target.split(',').flat_map(|c| c.parse()).collect();
            let machine_buttons: Vec<Vec<usize>> = machine_buttons
                .into_iter()
                .map(|button| button.split(',').flat_map(|d| d.parse::<usize>()).collect())
                .collect();

            let z3 = Optimize::new();
            let vars = (0..machine_buttons.len())
                .map(|button| Int::new_const(format!("b{}", button)))
                .collect::<Vec<_>>();

            for idx in 0..vars.len() {
                z3.assert(&vars[idx].ge(0));
            }

            let cols = (0..target.len())
                .map(|c| {
                    vars.iter()
                        .enumerate()
                        .filter(|(idx, _v)| machine_buttons[*idx].contains(&c))
                        .map(|(_idx, v)| v)
                        .sum::<Int>()
                })
                .collect::<Vec<_>>();

            for (idx, t) in target.iter().enumerate() {
                z3.assert(&cols[idx].eq(*t as u32));
            }
            let total = vars.iter().sum::<Int>();
            z3.minimize(&total);
            let _ = z3.check(&[]);
            let model = z3.get_model().unwrap();
            let result = model.eval(&total, true).unwrap().as_u64().unwrap();
            result as usize
        })
        .sum();

    println!("Day 10 Part 2 = {res}");
    res
}

#[test]
fn test_example() {
    let data = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    assert_eq!(solve1(data), 7);
    assert_eq!(solve2(data), 33);
}
