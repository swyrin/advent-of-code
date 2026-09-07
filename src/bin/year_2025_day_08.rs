use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use petgraph::unionfind::UnionFind;

fn main() {
    use std::io::Read;

    let mut input_content = std::fs::File::open("input.txt").expect("No input.txt file");
    let mut buffer = String::new();

    input_content.read_to_string(&mut buffer).unwrap();
    let input: Input = buffer.as_str().parse().unwrap();

    let ans1 = part_one(&input);
    let ans2 = part_two(&input);

    if let Ok(ans1) = ans1 {
        println!("Result of part 1: {ans1}")
    } else {
        println!("Part 1 fails.")
    }

    if let Ok(ans2) = ans2 {
        println!("Result of part 2: {ans2}")
    } else {
        println!("Part 2 fails.")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Point3 {
    fn distance_from(&self, other: &Self) -> isize {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }
}

struct Input {
    points: Vec<Point3>,
}

impl std::str::FromStr for Input {
    type Err = aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let points = parser!(lines(
            x:isize "," y:isize "," z:isize => Point3 { x, y, z }
        ))
        .parse(content)?;

        Ok(Self { points })
    }
}

fn sorted_edges(points: &[Point3]) -> Vec<(isize, (usize, usize))> {
    points
        .iter()
        .enumerate()
        .array_combinations()
        .map(|[(a_index, a), (b_index, b)]| (a.distance_from(b), (a_index, b_index)))
        .sorted_unstable_by_key(|(distance, _)| *distance)
        .collect()
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let points = &input.points;
    let edges = sorted_edges(points);
    let connection_count = if points.len() == 20 { 10 } else { 1000 };
    let mut components = UnionFind::new(points.len());

    for &(_, (a, b)) in edges.iter().take(connection_count) {
        components.union(a, b);
    }

    let mut component_sizes = HashMap::new();
    for point in 0..points.len() {
        *component_sizes
            .entry(components.find_mut(point))
            .or_insert(0_usize) += 1;
    }

    Ok(component_sizes
        .values()
        .sorted_unstable()
        .rev()
        .take(3)
        .product::<usize>())
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let points = &input.points;
    let edges = sorted_edges(points);
    let mut components = UnionFind::new(points.len());
    let mut component_count = points.len();

    for (_, (a, b)) in edges {
        if components.union(a, b) {
            component_count -= 1;
        }

        if component_count == 1 {
            return Ok(points[a].x * points[b].x);
        }
    }

    unreachable!("all points should eventually be connected")
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = { r"162,817,812
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
425,690,689" }, expected = { "40" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"162,817,812
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
425,690,689" }, expected = { "25272" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
