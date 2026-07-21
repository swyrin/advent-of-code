use std::collections::HashMap;

use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_libraries::core::aoc_input::AocInput;
use aoc_libraries::core::aoc_output::AocOutput;
use aoc_libraries::itertools::Itertools;
use aoc_libraries::petgraph::unionfind::UnionFind;
use aoc_macros::aoc_submission;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Point3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point3 {
    pub fn distance_from(&self, other: &Self) -> isize {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }
}

pub struct Input {
    pub points: Vec<Point3>,
}

impl AocInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let points = parser!(lines(
            x:isize "," y:isize "," z:isize => Point3 { x, y, z }
        ))
        .parse(content)
        .unwrap();

        Self { points }
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

#[aoc_submission(
    sample_in = r"162,817,812
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
425,690,689",
    sample_out = 40
)]
pub fn part_1(input: Input) -> AocOutput {
    let points = input.points;
    let edges = sorted_edges(&points);
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

    AocOutput::from_number(
        component_sizes
            .values()
            .sorted_unstable()
            .rev()
            .take(3)
            .product::<usize>(),
    )
}

#[aoc_submission(
    sample_in = r"162,817,812
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
425,690,689",
    sample_out = 25272
)]
pub fn part_2(input: Input) -> AocOutput {
    let points = input.points;
    let edges = sorted_edges(&points);
    let mut components = UnionFind::new(points.len());
    let mut component_count = points.len();

    for (_, (a, b)) in edges {
        if components.union(a, b) {
            component_count -= 1;
        }

        if component_count == 1 {
            return AocOutput::from_number(points[a].x * points[b].x);
        }
    }

    unreachable!("all points should eventually be connected")
}
