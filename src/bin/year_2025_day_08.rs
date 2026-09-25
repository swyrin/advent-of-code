use std::collections::HashMap;

use itertools::Itertools;
use macros::{AocInput, aoc, part, sample};
use petgraph::unionfind::UnionFind;

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

#[derive(AocInput)]
struct Input {
    #[parse(lines(
        x:isize "," y:isize "," z:isize => Point3 { x, y, z }
    ))]
    pub(crate) points: Vec<Point3>,
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
aoc!();

#[part]
#[sample(
    input = "162,817,812
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
    expected = "40"
)]
fn part_one(
    Input {
        points,
    }: &Input,
) -> impl std::fmt::Display {
    let edges = sorted_edges(points);
    let connection_count = if points.len() == 20 { 10 } else { 1000 };
    let mut components = UnionFind::new(points.len());

    for &(_, (a, b)) in edges.iter().take(connection_count) {
        components.union(a, b);
    }

    let mut component_sizes = HashMap::new();
    for point in 0..points.len() {
        *component_sizes.entry(components.find_mut(point)).or_insert(0_usize) += 1;
    }

    component_sizes.values().sorted_unstable().rev().take(3).product::<usize>()
}

#[part]
#[sample(
    input = "162,817,812
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
    expected = "25272"
)]
fn part_two(
    Input {
        points,
    }: &Input,
) -> impl std::fmt::Display {
    let edges = sorted_edges(points);
    let mut components = UnionFind::new(points.len());
    let mut component_count = points.len();

    for (_, (a, b)) in edges {
        if components.union(a, b) {
            component_count -= 1;
        }

        if component_count == 1 {
            return points[a].x * points[b].x;
        }
    }

    unreachable!("all points should eventually be connected")
}
