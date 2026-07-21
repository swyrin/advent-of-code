use std::collections::{HashMap, HashSet};

use aoc_libraries::itertools::Itertools;
use aoc_libraries::pathfinding::matrix::Matrix;
use aoc_macros::aoc_submission;

pub type Position = (isize, isize);

pub struct Input {
    pub rows: isize,
    pub columns: isize,
    pub antennas: HashMap<char, Vec<Position>>,
}

impl std::str::FromStr for Input {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let grid = Matrix::from_rows(
            content
                .lines()
                .filter(|line| !line.is_empty())
                .map(str::chars),
        )
        .map_err(|error| format!("invalid antenna map: {error:?}"))?;
        let mut antennas = HashMap::<char, Vec<Position>>::new();

        for (row, column) in grid.keys() {
            let frequency = grid[(row, column)];
            if frequency.is_ascii_alphanumeric() {
                antennas
                    .entry(frequency)
                    .or_default()
                    .push((row as isize, column as isize));
            }
        }

        Ok(Self {
            rows: grid.rows as isize,
            columns: grid.columns as isize,
            antennas,
        })
    }
}

impl Input {
    fn in_bounds(&self, (row, column): Position) -> bool {
        (0..self.rows).contains(&row) && (0..self.columns).contains(&column)
    }
}

#[aoc_submission(
    sample_in = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    sample_out = 14
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    let mut antinodes = HashSet::new();

    for locations in input.antennas.values() {
        for [first, second] in locations.iter().copied().array_combinations() {
            let delta = (second.0 - first.0, second.1 - first.1);
            antinodes.insert((first.0 - delta.0, first.1 - delta.1));
            antinodes.insert((second.0 + delta.0, second.1 + delta.1));
        }
    }

    antinodes
        .into_iter()
        .filter(|&position| input.in_bounds(position))
        .count()
}

#[aoc_submission(
    sample_in = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    sample_out = 34
)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    let mut antinodes = HashSet::new();

    for locations in input.antennas.values() {
        for [first, second] in locations.iter().copied().array_combinations() {
            let delta = (second.0 - first.0, second.1 - first.1);

            let mut position = first;
            while input.in_bounds(position) {
                antinodes.insert(position);
                position = (position.0 - delta.0, position.1 - delta.1);
            }

            let mut position = second;
            while input.in_bounds(position) {
                antinodes.insert(position);
                position = (position.0 + delta.0, position.1 + delta.1);
            }
        }
    }

    antinodes.len()
}
