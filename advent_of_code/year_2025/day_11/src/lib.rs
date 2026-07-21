use std::collections::HashMap;
use std::hash::RandomState;

use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_libraries::pathfinding::prelude::count_paths;
use aoc_libraries::petgraph::algo::all_simple_paths;
use aoc_libraries::petgraph::graph::{DiGraph, NodeIndex};
use aoc_macros::aoc_submission;

#[derive(Debug)]
pub struct Adjacent {
    pub from: String,
    pub neighbors: Vec<String>,
}

pub struct Input {
    pub entries: Vec<Adjacent>,
}

impl std::str::FromStr for Input {
    type Err = aoc_libraries::aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let entries = parser!(lines(
            from:string(alnum+) ": " neighbors:repeat_sep(string(alnum+), " ")
                => Adjacent { from, neighbors }
        ))
        .parse(content)?;

        Ok(Self { entries })
    }
}

#[aoc_submission(
    sample_in = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
    sample_out = 5
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    let mut graph = DiGraph::<String, ()>::new();
    let mut node_indices: HashMap<String, NodeIndex> = HashMap::new();

    for entry in &input.entries {
        let parent_index = *node_indices
            .entry(entry.from.clone())
            .or_insert_with(|| graph.add_node(entry.from.clone()));

        for neighbor in &entry.neighbors {
            let child_index = *node_indices
                .entry(neighbor.clone())
                .or_insert_with(|| graph.add_node(neighbor.clone()));
            graph.add_edge(parent_index, child_index, ());
        }
    }

    let you = *node_indices.get("you").unwrap();
    let out = *node_indices.get("out").unwrap();
    all_simple_paths::<Vec<_>, _, RandomState>(&graph, you, out, 0, None).count()
}

#[aoc_submission(
    sample_in = r"svr: aaa bbb
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
hhh: out",
    sample_out = 2
)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    let connections: HashMap<_, _> = input
        .entries
        .into_iter()
        .map(|entry| (entry.from, entry.neighbors))
        .collect();

    count_paths(
        ("svr".to_string(), false, false),
        |(current, seen_dac, seen_fft)| {
            connections
                .get(current)
                .into_iter()
                .flatten()
                .map(|next| {
                    (
                        next.clone(),
                        *seen_dac || next == "dac",
                        *seen_fft || next == "fft",
                    )
                })
                .collect::<Vec<_>>()
        },
        |(current, seen_dac, seen_fft)| current == "out" && *seen_dac && *seen_fft,
    )
}
