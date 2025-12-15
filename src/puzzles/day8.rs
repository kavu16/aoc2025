use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    error::Error,
};

use itertools::Itertools;

pub fn solve1(data: &str, min_connections: usize) -> usize {
    let points: HashMap<Point, usize> = data
        .lines()
        .flat_map(|line| line.try_into())
        .enumerate()
        .map(|(id, p)| (p, id))
        .collect();

    let mut distance_heap = BinaryHeap::new();

    for pair in points.keys().combinations(2) {
        let (point1, point2) = (pair[0], pair[1]);
        let distance = point1.distance_squared(point2);
        distance_heap.push((Reverse(distance), point1, point2));
    }

    let mut uf = UnionFind::new(points.len());

    let mut connections = 0;
    while let Some((_, point1, point2)) = distance_heap.pop()
        && connections < min_connections
    {
        let (id1, id2) = (points[&point1], points[&point2]);
        connections += 1;
        uf.union(id1, id2);
    }

    let res = uf
        .ranks
        .into_iter()
        .enumerate()
        .filter_map(|(idx, rank)| {
            if uf.parents[idx] == idx {
                Some(rank)
            } else {
                None
            }
        })
        .sorted()
        .rev()
        .take(3)
        .product();

    println!("Day 8 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> usize {
    let points: HashMap<Point, usize> = data
        .lines()
        .flat_map(|line| line.try_into())
        .enumerate()
        .map(|(id, p)| (p, id))
        .collect();

    let mut distance_heap = BinaryHeap::new();

    for pair in points.keys().combinations(2) {
        let (point1, point2) = (pair[0], pair[1]);
        let distance = point1.distance_squared(point2);
        distance_heap.push((Reverse(distance), point1, point2));
    }

    let mut uf = UnionFind::new(points.len());

    while let Some((_, point1, point2)) = distance_heap.pop() {
        let (id1, id2) = (points[&point1], points[&point2]);
        uf.union(id1, id2);
        if uf.size == 1 {
            let res = point1.0 * point2.0;
            println!("Day 8 Part 2 = {res}");
            return res;
        }
    }
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(usize, usize, usize);

impl TryFrom<&str> for Point {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Box<dyn Error>> {
        match value.split(',').collect::<Vec<&str>>().as_slice() {
            [x, y, z] => Ok(Point(x.parse()?, y.parse()?, z.parse()?)),
            _ => Err("Invalid input".into()),
        }
    }
}

impl Point {
    pub fn distance_squared(&self, other: &Point) -> usize {
        let (dx, dy, dz) = (
            self.0.abs_diff(other.0),
            self.1.abs_diff(other.1),
            self.2.abs_diff(other.2),
        );
        dx * dx + dy * dy + dz * dz
    }
}

struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            ranks: vec![1; size],
            size,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            self.parents[x] = self.find(self.parents[x]);
        }
        self.parents[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        self.size -= 1;
        if self.ranks[root_x] < self.ranks[root_y] {
            self.parents[root_x] = root_y;
            self.ranks[root_y] += self.ranks[root_x];
        } else {
            self.parents[root_y] = root_x;
            self.ranks[root_x] += self.ranks[root_y];
        }
    }
}

#[test]
fn test_example() {
    let data = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    assert_eq!(solve1(data, 10), 40);
    assert_eq!(solve2(data), 25272);
}
