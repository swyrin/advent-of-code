use std::collections::HashMap;
use std::hash::RandomState;

use aoc_parse::{parser, prelude::*};
use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, NodeIndex};

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

#[derive(Debug)]
struct Adjacent {
    from: String,
    neighbors: Vec<String>,
}

struct Input {
    entries: Vec<Adjacent>,
}

impl std::str::FromStr for Input {
    type Err = aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let entries = parser!(lines(
            from:string(alnum+) ": " neighbors:repeat_sep(string(alnum+), " ")
                => Adjacent { from, neighbors }
        ))
        .parse(content)?;

        Ok(Self { entries })
    }
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
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

    Ok(all_simple_paths::<Vec<_>, _, RandomState>(&graph, you, out, 0, None).count())
}

fn count_routes(
    connections: &HashMap<String, Vec<String>>,
    state: (String, bool, bool),
    memo: &mut HashMap<(String, bool, bool), u128>,
) -> u128 {
    if let Some(&count) = memo.get(&state) {
        return count;
    }

    let (current, seen_dac, seen_fft) = state;
    let count = if current == "out" && seen_dac && seen_fft {
        1
    } else {
        connections
            .get(&current)
            .into_iter()
            .flatten()
            .map(|next| {
                count_routes(
                    connections,
                    (
                        next.clone(),
                        seen_dac || next == "dac",
                        seen_fft || next == "fft",
                    ),
                    memo,
                )
            })
            .sum()
    };

    memo.insert((current, seen_dac, seen_fft), count);
    count
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let connections: HashMap<_, _> = input
        .entries
        .iter()
        .map(|entry| (entry.from.clone(), entry.neighbors.clone()))
        .collect();

    Ok(count_routes(
        &connections,
        ("svr".to_string(), false, false),
        &mut HashMap::new(),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = { r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out" }, expected = { "5" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"svr: aaa bbb
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
hhh: out" }, expected = { "2" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
