use std::collections::HashSet;

use aoc_libraries::pathfinding::matrix::Matrix;
use aoc_macros::aoc_submission;

pub type Position = (isize, isize);

pub struct Input {
    pub garden: Matrix<u8>,
}

impl std::str::FromStr for Input {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let garden = Matrix::from_rows(
            content
                .lines()
                .filter(|line| !line.is_empty())
                .map(str::bytes),
        )
        .map_err(|error| format!("invalid garden map: {error:?}"))?;

        Ok(Self { garden })
    }
}

impl Input {
    fn plant(&self, (row, column): Position) -> Option<u8> {
        let row = usize::try_from(row).ok()?;
        let column = usize::try_from(column).ok()?;
        self.garden.get((row, column)).copied()
    }

    fn regions(&self) -> Vec<HashSet<Position>> {
        const DIRECTIONS: [Position; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut visited = HashSet::new();
        let mut regions = Vec::new();

        for (row, column) in self.garden.keys() {
            let start = (row as isize, column as isize);
            if !visited.insert(start) {
                continue;
            }

            let plant = self.garden[(row, column)];
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
    const CORNERS: [(Position, Position); 4] = [
        ((-1, 0), (0, -1)),
        ((-1, 0), (0, 1)),
        ((1, 0), (0, -1)),
        ((1, 0), (0, 1)),
    ];

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

#[aoc_submission(
    sample_in = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    sample_out = 1930
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    input
        .regions()
        .into_iter()
        .map(|region| region.len() * perimeter(&region))
        .sum::<usize>()
}

#[aoc_submission(
    sample_in = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    sample_out = 1206
)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    input
        .regions()
        .into_iter()
        .map(|region| region.len() * side_count(&region))
        .sum::<usize>()
}
