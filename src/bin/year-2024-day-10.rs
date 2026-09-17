use std::collections::HashMap;

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Bfs;

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

struct Input {
    graph: DiGraph<u8, ()>,
}

impl std::str::FromStr for Input {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let rows = content
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|height| {
                        height
                            .to_digit(10)
                            .map(|height| height as u8)
                            .ok_or_else(|| format!("invalid trail height: {height}"))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if rows.is_empty() {
            return Err("invalid topographic map: empty".to_string());
        }

        if rows.iter().any(|row| row.len() != rows[0].len()) {
            return Err("invalid topographic map: ragged rows".to_string());
        }

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
                    let Some(&neighbour_height) = rows
                        .get(neighbour_row)
                        .and_then(|line| line.get(neighbour_column))
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

        Ok(Self { graph })
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
        graph
            .neighbors(node)
            .map(|next| count_trails(graph, next, memo))
            .sum()
    };

    memo.insert(node, count);
    count
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut total = 0;

    for trailhead in input
        .graph
        .node_indices()
        .filter(|&node| input.graph[node] == 0)
    {
        let mut search = Bfs::new(&input.graph, trailhead);
        let mut count = 0;

        while let Some(node) = search.next(&input.graph) {
            if input.graph[node] == 9 {
                count += 1;
            }
        }

        total += count;
    }

    Ok(total)
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut memo = HashMap::new();

    Ok(input
        .graph
        .node_indices()
        .filter(|&node| input.graph[node] == 0)
        .map(|trailhead| count_trails(&input.graph, trailhead, &mut memo))
        .sum::<u64>())
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = { r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732" }, expected = { "36" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732" }, expected = { "81" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
