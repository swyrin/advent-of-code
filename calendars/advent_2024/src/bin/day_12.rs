use std::collections::HashSet;

use macros::{AocInput, aoc, part, sample};

type Position = (isize, isize);

#[derive(AocInput)]
struct Input {
    #[parse(rows:lines(string(any_char+)) => to_garden(rows))]
    garden: Vec<Vec<u8>>,
}

fn to_garden(rows: Vec<String>) -> Vec<Vec<u8>> {
    let garden = rows
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    if garden.is_empty() {
        panic!("invalid garden map: empty");
    }

    if garden.iter().any(|row| row.len() != garden[0].len()) {
        panic!("invalid garden map: ragged rows");
    }

    garden
}

impl Input {
    fn plant(&self, (row, column): Position) -> Option<u8> {
        if row < 0 || column < 0 {
            return None;
        }

        self.garden.get(row as usize).and_then(|line| line.get(column as usize)).copied()
    }

    fn regions(&self) -> Vec<HashSet<Position>> {
        const DIRECTIONS: [Position; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut visited = HashSet::new();
        let mut regions = Vec::new();

        for row in 0..self.garden.len() {
            for column in 0..self.garden[row].len() {
                let start = (row as isize, column as isize);
                if !visited.insert(start) {
                    continue;
                }

                let plant = self.garden[row][column];
                let mut region = HashSet::from([start]);
                let mut stack = vec![start];

                while let Some(position) = stack.pop() {
                    for direction in DIRECTIONS {
                        let neighbour = (position.0 + direction.0, position.1 + direction.1);
                        if self.plant(neighbour) == Some(plant) && visited.insert(neighbour) {
                            region.insert(neighbour);
                            stack.push(neighbour);
                        }
                    }
                }

                regions.push(region);
            }
        }

        regions
    }
}

fn perimeter(region: &HashSet<Position>) -> usize {
    const DIRECTIONS: [Position; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    region
        .iter()
        .map(|&(row, column)| {
            DIRECTIONS
                .iter()
                .filter(|&&(delta_row, delta_column)| {
                    !region.contains(&(row + delta_row, column + delta_column))
                })
                .count()
        })
        .sum()
}

fn side_count(region: &HashSet<Position>) -> usize {
    const CORNERS: [(Position, Position); 4] =
        [((-1, 0), (0, -1)), ((-1, 0), (0, 1)), ((1, 0), (0, -1)), ((1, 0), (0, 1))];

    region
        .iter()
        .map(|&(row, column)| {
            CORNERS
                .iter()
                .filter(|&&(first, second)| {
                    let first = (row + first.0, column + first.1);
                    let second = (row + second.0, column + second.1);
                    let diagonal = (first.0 + second.0 - row, first.1 + second.1 - column);
                    let has_first = region.contains(&first);
                    let has_second = region.contains(&second);

                    (!has_first && !has_second)
                        || (has_first && has_second && !region.contains(&diagonal))
                })
                .count()
        })
        .sum()
}
aoc!();

#[part]
#[sample(
    input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    expected = "1930"
)]
fn part_one(
    input @ Input {
        ..
    }: &Input,
) -> impl std::fmt::Display {
    input.regions().into_iter().map(|region| region.len() * perimeter(&region)).sum::<usize>()
}

#[part]
#[sample(
    input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    expected = "1206"
)]
fn part_two(
    input @ Input {
        ..
    }: &Input,
) -> impl std::fmt::Display {
    input.regions().into_iter().map(|region| region.len() * side_count(&region)).sum::<usize>()
}
