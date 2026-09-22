use std::collections::HashMap;

use macros::{AocInput, aoc};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Bfs;

#[derive(AocInput)]
struct Input {
    #[parse(rows:lines(string(any_char+)) => to_heights(rows))]
    heights: Vec<Vec<u8>>,
}

fn to_heights(rows: Vec<String>) -> Vec<Vec<u8>> {
    let heights = rows
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|height| {
                    height
                        .to_digit(10)
                        .map(|height| height as u8)
                        .unwrap_or_else(|| panic!("invalid trail height: {height}"))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if heights.is_empty() {
        panic!("invalid topographic map: empty");
    }

    if heights.iter().any(|row| row.len() != heights[0].len()) {
        panic!("invalid topographic map: ragged rows");
    }

    heights
}

impl Input {
    fn graph(&self) -> DiGraph<u8, ()> {
        let rows = &self.heights;

        let mut graph = DiGraph::<u8, ()>::new();
        let mut nodes = vec![vec![NodeIndex::new(0); rows[0].len()]; rows.len()];

        for (row, line) in rows.iter().enumerate() {
            for (column, &height) in line.iter().enumerate() {
                nodes[row][column] = graph.add_node(height);
            }
        }

        const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (row, line) in rows.iter().enumerate() {
            for (column, &height) in line.iter().enumerate() {
                for &(delta_row, delta_column) in &DIRECTIONS {
                    let neighbour_row = row as isize + delta_row;
                    let neighbour_column = column as isize + delta_column;

                    let Some(neighbour_row) = usize::try_from(neighbour_row).ok() else {
                        continue;
                    };
                    let Some(neighbour_column) = usize::try_from(neighbour_column).ok() else {
                        continue;
                    };
                    let Some(&neighbour_height) =
                        rows.get(neighbour_row).and_then(|line| line.get(neighbour_column))
                    else {
                        continue;
                    };

                    if neighbour_height == height + 1 {
                        graph.add_edge(
                            nodes[row][column],
                            nodes[neighbour_row][neighbour_column],
                            (),
                        );
                    }
                }
            }
        }

        graph
    }
}

fn count_trails(
    graph: &DiGraph<u8, ()>,
    node: NodeIndex,
    memo: &mut HashMap<NodeIndex, u64>,
) -> u64 {
    if let Some(&count) = memo.get(&node) {
        return count;
    }

    let count = if graph[node] == 9 {
        1
    } else {
        graph.neighbors(node).map(|next| count_trails(graph, next, memo)).sum()
    };

    memo.insert(node, count);
    count
}

aoc! {
    #[sample(
        input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        expected = "36"
    )]
    fn part_one(input @ Input { .. }: &Input) -> impl std::fmt::Display {
        let graph = input.graph();
        let mut total = 0;

        for trailhead in graph.node_indices().filter(|&node| graph[node] == 0) {
            let mut search = Bfs::new(&graph, trailhead);
            let mut count = 0;

            while let Some(node) = search.next(&graph) {
                if graph[node] == 9 {
                    count += 1;
                }
            }

            total += count;
        }

        total
    }

    #[sample(
        input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        expected = "81"
    )]
    fn part_two(input @ Input { .. }: &Input) -> impl std::fmt::Display {
        let graph = input.graph();
        let mut memo = HashMap::new();

        graph
            .node_indices()
            .filter(|&node| graph[node] == 0)
            .map(|trailhead| count_trails(&graph, trailhead, &mut memo))
            .sum::<u64>()
    }
}
