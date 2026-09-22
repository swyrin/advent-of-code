use std::collections::HashMap;
use std::hash::RandomState;

use macros::{AocInput, aoc};
use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, NodeIndex};

#[derive(Debug, Clone)]
struct Adjacent {
    from: String,
    neighbors: Vec<String>,
}

#[derive(AocInput)]
struct Input {
    #[parse(lines(
        from:string(alnum+) ": " neighbors:repeat_sep(string(alnum+), " ")
            => Adjacent { from, neighbors }
    ))]
    pub(crate) entries: Vec<Adjacent>,
}

aoc! {
    #[sample(
        input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
        expected = "5"
    )]
    fn part_one(Input { entries }: &Input) -> impl std::fmt::Display {
        let mut graph = DiGraph::<String, ()>::new();
        let mut node_indices: HashMap<String, NodeIndex> = HashMap::new();

        for entry in entries {
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

    #[sample(
        input = "svr: aaa bbb
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
        expected = "2"
    )]
    fn part_two(Input { entries }: &Input) -> impl std::fmt::Display {
        let connections: HashMap<_, _> =
            entries.iter().map(|entry| (entry.from.clone(), entry.neighbors.clone())).collect();

        count_routes(&connections, ("svr".to_string(), false, false), &mut HashMap::new())
    }
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
                    (next.clone(), seen_dac || next == "dac", seen_fft || next == "fft"),
                    memo,
                )
            })
            .sum()
    };

    memo.insert((current, seen_dac, seen_fft), count);
    count
}
