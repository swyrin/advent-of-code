use std::collections::{HashMap, HashSet};
use std::hash::RandomState;

use aoc_libraries::petgraph::algo::all_simple_paths;
use aoc_libraries::petgraph::graph::{DiGraph, NodeIndex};
use aoc_macros::aoc_submission;
use aoc_utils::traits::generalised_output::UmiAteTheOutput;
use aoc_utils::traits::parsable_input::ParsableInput;

#[derive(Debug)]
pub struct Adjacent {
    pub from: String,
    pub neighbors: Vec<String>,
}

pub struct Input {
    pub entries: Vec<Adjacent>,
}

impl ParsableInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let entries = content
            .lines()
            .map(|line| {
                let mut components = line.split_whitespace();
                let from = components.next().unwrap().trim_end_matches(':').to_string();
                let neighbors = components.map(str::to_string).collect();
                Adjacent { from, neighbors }
            })
            .collect();

        Self { entries }
    }
}

#[aoc_submission(
    input_type = crate::Input,
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
pub fn part_1(input: Input) -> UmiAteTheOutput {
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
    let path_count = all_simple_paths::<Vec<_>, _, RandomState>(&graph, you, out, 0, None).count();

    UmiAteTheOutput::from_number(path_count)
}

fn find_paths(
    start: &str,
    connections: &HashMap<String, HashSet<String>>,
    cache: &mut HashMap<String, (usize, usize, usize, usize)>,
) -> (usize, usize, usize, usize) {
    if start == "out" {
        return (0, 0, 0, 1);
    }

    if let Some(result) = cache.get(start) {
        return *result;
    }

    let (mut dac_paths, mut fft_paths, mut both_paths, mut total_paths) = (0, 0, 0, 0);

    if let Some(connected_devices) = connections.get(start) {
        for connected_device in connected_devices {
            let (child_dac_paths, child_fft_paths, child_both_paths, child_total_paths) =
                find_paths(connected_device, connections, cache);

            dac_paths += if start == "dac" {
                child_total_paths
            } else {
                child_dac_paths
            };
            fft_paths += if start == "fft" {
                child_total_paths
            } else {
                child_fft_paths
            };
            both_paths += match start {
                "dac" => child_fft_paths,
                "fft" => child_dac_paths,
                _ => child_both_paths,
            };
            total_paths += child_total_paths;
        }
    }

    let result = (dac_paths, fft_paths, both_paths, total_paths);
    cache.insert(start.to_owned(), result);
    result
}

#[aoc_submission(
    input_type = crate::Input,
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
pub fn part_2(input: Input) -> UmiAteTheOutput {
    let connections = input
        .entries
        .into_iter()
        .map(|entry| (entry.from, HashSet::from_iter(entry.neighbors)))
        .collect();

    UmiAteTheOutput::from_number(find_paths("svr", &connections, &mut HashMap::new()).2)
}
