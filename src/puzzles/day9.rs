use std::collections::HashMap;

use itertools::Itertools;

pub fn solve1(data: &str) -> usize {
    let res = data
        .lines()
        .flat_map(|line| Point::try_from(line))
        .combinations(2)
        .map(|pair| pair[0].rect_area(&pair[1]))
        .max()
        .unwrap_or(0);
    println!("Day 9 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> usize {
    let mut vertical: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut horizontal: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    let points: Vec<_> = data.lines().flat_map(|line| line.try_into()).collect();

    for (Point(x0, y0), Point(x1, y1)) in points.iter().copied().circular_tuple_windows() {
        match (x0.cmp(&x1), y0.cmp(&y1)) {
            (std::cmp::Ordering::Equal, _) => {
                vertical
                    .entry(x0)
                    .or_default()
                    .push((y0.min(y1), y0.max(y1)));
            }
            (_, std::cmp::Ordering::Equal) => {
                horizontal
                    .entry(y0)
                    .or_default()
                    .push((x0.min(x1), x0.max(x1)));
            }
            _ => {}
        }
    }
    // println!("{horizontal:?}");
    // println!("{vertical:?}");

    let res = points
        .into_iter()
        .combinations(2)
        .fold(0usize, |max_area, pair| {
            let Point(x0, y0) = pair[0];
            let Point(x1, y1) = pair[1];
            let (y_min, y_max) = (y0.min(y1), y0.max(y1));
            let (x_min, x_max) = (x0.min(x1), x0.max(x1));
            for y in (y_min + 1)..y_max {
                if let Some(horizontals) = horizontal.get(&y)
                    && horizontals.iter().any(|(h_left, h_right)| {
                        (h_left..h_right).contains(&&x_min)
                            || ((h_left + 1)..=*h_right).contains(&&x_max)
                    })
                {
                    return max_area;
                }
            }
            for x in (x_min + 1)..x_max {
                if let Some(verticals) = vertical.get(&x)
                    && verticals.iter().any(|(v_bottom, v_top)| {
                        (v_bottom..v_top).contains(&&y_min)
                            || ((v_bottom + 1)..=*v_top).contains(&&y_max)
                    })
                {
                    return max_area;
                }
            }
            max_area.max(pair[0].rect_area(&pair[1]))
        });

    println!("Day 9 Part 2 = {res}");
    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl TryFrom<&str> for Point {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (x, y) = value.split_once(',').ok_or("invalid input")?;
        Ok(Point(x.parse()?, y.parse()?))
    }
}

impl Point {
    fn rect_area(&self, other: &Point) -> usize {
        let width = self.0.abs_diff(other.0) + 1;
        let height = self.1.abs_diff(other.1) + 1;
        width * height
    }
}

#[test]
fn test_example() {
    let data = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    assert_eq!(solve1(data), 50);
    assert_eq!(solve2(data), 24);
}
