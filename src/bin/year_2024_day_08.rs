use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use macros::{AocInput, aoc};

type Position = (isize, isize);

#[derive(AocInput)]
struct Input {
    #[parse(rows:lines(string(any_char+)) => to_grid(rows))]
    pub(crate) grid: Vec<Vec<char>>,
}

fn to_grid(rows: Vec<String>) -> Vec<Vec<char>> {
    let grid = rows
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    if grid.is_empty() {
        panic!("invalid antenna map: empty");
    }

    if grid.iter().any(|row| row.len() != grid[0].len()) {
        panic!("invalid antenna map: ragged rows");
    }

    grid
}

impl Input {
    fn rows(&self) -> isize {
        self.grid.len() as isize
    }

    fn columns(&self) -> isize {
        self.grid[0].len() as isize
    }

    fn antennas(&self) -> HashMap<char, Vec<Position>> {
        let mut antennas = HashMap::<char, Vec<Position>>::new();

        for (row, line) in self.grid.iter().enumerate() {
            for (column, &frequency) in line.iter().enumerate() {
                if frequency.is_ascii_alphanumeric() {
                    antennas.entry(frequency).or_default().push((row as isize, column as isize));
                }
            }
        }

        antennas
    }

    fn in_bounds(&self, (row, column): Position) -> bool {
        (0..self.rows()).contains(&row) && (0..self.columns()).contains(&column)
    }
}

aoc! {
    #[sample(
        input = "............
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
        expected = "14"
    )]
    fn part_one(input @ Input { .. }: &Input) -> impl std::fmt::Display {
        let mut antinodes = HashSet::new();
        let antennas = input.antennas();

        for locations in antennas.values() {
            for [first, second] in locations.iter().copied().array_combinations() {
                let delta = (second.0 - first.0, second.1 - first.1);
                antinodes.insert((first.0 - delta.0, first.1 - delta.1));
                antinodes.insert((second.0 + delta.0, second.1 + delta.1));
            }
        }

        antinodes.into_iter().filter(|&position| input.in_bounds(position)).count()
    }

    #[sample(
        input = "............
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
        expected = "34"
    )]
    fn part_two(input @ Input { .. }: &Input) -> impl std::fmt::Display {
        let mut antinodes = HashSet::new();
        let antennas = input.antennas();

        for locations in antennas.values() {
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
}
