use std::collections::BTreeMap;

use aoc_libraries::petgraph;
use aoc_libraries::petgraph::algo;
use aoc_libraries::petgraph::graph::UnGraph;
use aoc_macros::aoc_submission;
use aoc_utils::traits::generalised_output::UmiAteTheOutput;
use aoc_utils::traits::parsable_input::ParsableInput;

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

    pub fn from_line(line: &str) -> Self {
        let numbers: Vec<&str> = line.trim().split(',').take(3).collect();

        Self {
            x: numbers[0].parse().unwrap(),
            y: numbers[1].parse().unwrap(),
            z: numbers[2].parse().unwrap(),
        }
    }
}

pub struct Input {
    pub points: Vec<Point3>,
}

impl ParsableInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let points = content.lines().map(Point3::from_line).collect();
        Self { points }
    }
}

#[aoc_submission(
    input_type = crate::Input,
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
pub fn part_1(input: Input) -> UmiAteTheOutput {
    let points = input.points;
    let mut edges = BTreeMap::<isize, (u32, u32)>::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = points[i].distance_from(&points[j]);
            edges.insert(distance, (i as u32, j as u32));
        }
    }

    // The puzzle sample asks for 10 connections; the full input asks for 1000.
    let connection_count = if points.len() == 20 { 10 } else { 1000 };
    let graph = UnGraph::<u32, ()>::from_edges(edges.values().take(connection_count));

    let mut component_sizes: Vec<usize> = petgraph::algo::kosaraju_scc(&graph)
        .iter()
        .map(|component| component.len())
        .collect();

    component_sizes.sort_unstable();

    UmiAteTheOutput::from_number(component_sizes.iter().rev().take(3).product::<usize>())
}

#[aoc_submission(
    input_type = crate::Input,
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
pub fn part_2(input: Input) -> UmiAteTheOutput {
    let points = input.points;
    let mut graph = UnGraph::<u32, ()>::new_undirected();
    let mut edges = BTreeMap::<isize, (u32, u32)>::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = points[i].distance_from(&points[j]);
            edges.insert(distance, (i as u32, j as u32));
        }
    }

    for i in 0..points.len() {
        graph.add_node(i as u32);
    }

    loop {
        let edge = edges.pop_first().unwrap().1;
        graph.add_edge(edge.0.into(), edge.1.into(), ());

        if algo::connected_components(&graph) == 1 {
            let a = points[edge.0 as usize].x;
            let b = points[edge.1 as usize].x;
            return UmiAteTheOutput::from_number(b * a);
        }
    }
}
