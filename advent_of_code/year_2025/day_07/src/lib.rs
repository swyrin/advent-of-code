use std::collections::{HashMap, HashSet};

use aoc_macros::aoc_submission;

pub struct Input {
    pub content: String,
}

impl std::str::FromStr for Input {
    type Err = std::convert::Infallible;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            content: content.to_string(),
        })
    }
}

#[aoc_submission(
    sample_in = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
    sample_out = 21
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    let mut required_columns = HashSet::new();
    let lines = input.content.lines();

    for line in lines.clone().take(1) {
        for (column, character) in line.chars().enumerate() {
            if character == 'S' {
                required_columns.insert(column);
                break;
            }
        }
    }

    let mut total = 0_u128;

    for line in lines.skip(2).step_by(2) {
        let mut processed_columns = HashSet::new();
        let mut next_required_columns = HashSet::new();

        for (column, character) in line.chars().enumerate() {
            if character == '^' && required_columns.contains(&column) {
                processed_columns.insert(column);
                total += 1;
                next_required_columns.insert(column - 1);
                next_required_columns.insert(column + 1);
            }
        }

        for column in processed_columns {
            required_columns.remove(&column);
        }

        required_columns.extend(next_required_columns);
    }

    total
}

#[aoc_submission(
    sample_in = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
    sample_out = 40
)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    // how many rays reach x
    let mut rays: HashMap<usize, u128> = HashMap::new();
    let lines = input.content.lines();

    for line in lines.clone().take(1) {
        for (column, character) in line.chars().enumerate() {
            if character == 'S' {
                rays.insert(column, 1);
                break;
            }
        }
    }

    for line in lines.skip(2).step_by(2) {
        let mut next_rays = HashMap::new();

        for (column, character) in line.chars().enumerate() {
            if let Some(ray_count) = rays.get(&column) {
                if character == '^' {
                    *next_rays.entry(column + 1).or_insert(0) += ray_count;
                    *next_rays.entry(column - 1).or_insert(0) += ray_count;
                } else {
                    *next_rays.entry(column).or_insert(0) += ray_count;
                }
            }
        }

        rays = next_rays;
    }

    rays.values().copied().sum::<u128>()
}
