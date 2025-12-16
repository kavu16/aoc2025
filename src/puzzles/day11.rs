use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve1(data: &str) -> usize {
    let adj: HashMap<&str, Vec<&str>> = data
        .lines()
        .map(|line| {
            let (node, neighbors) = line.split_once(": ").unwrap();
            (node, neighbors.split_whitespace().collect())
        })
        .collect();

    let mut queue = VecDeque::from([("you", HashSet::from(["you"]))]);
    let mut paths = 0;
    while let Some((curr, path)) = queue.pop_front() {
        if curr == "out" {
            paths += 1;
            continue;
        }
        for neighbor in adj[curr].iter() {
            if !path.contains(neighbor) {
                let mut new_path = path.clone();
                new_path.insert(neighbor);
                queue.push_front((neighbor, new_path));
            }
        }
    }

    let res = paths;
    println!("Day 11 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> usize {
    let adj: HashMap<&str, Vec<&str>> = data
        .lines()
        .map(|line| {
            let (node, neighbors) = line.split_once(": ").unwrap();
            (node, neighbors.split_whitespace().collect())
        })
        .collect();

    let mut cache = HashMap::new();
    let paths = HashSet::from(["svr"]);

    let res = check_paths("svr", &paths, &adj, &mut cache);
    println!("Day 11 Part 2 = {res}");
    res
}

fn check_paths<'a>(
    node: &'a str,
    path: &HashSet<&'a str>,
    adj: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<(&'a str, bool, bool), usize>,
) -> usize {
    if let Some(&count) = cache.get(&(node, path.contains("fft"), path.contains("dac"))) {
        return count;
    }
    if node == "out" && path.contains("fft") && path.contains("dac") {
        return 1;
    }

    let mut count = 0;
    if let Some(neighbors) = adj.get(node) {
        for neighbor in neighbors {
            if path.contains(neighbor) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.insert(neighbor);
            count += check_paths(neighbor, &new_path, adj, cache);
        }
    }
    cache.insert((node, path.contains("fft"), path.contains("dac")), count);
    count
}

#[test]
fn test_example() {
    let data1 = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    let data2 = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    assert_eq!(solve1(data1), 5);
    assert_eq!(solve2(data2), 2);
}
