use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_libraries::counter::Counter;
use aoc_macros::aoc_submission;

pub struct Input {
    pub left_values: Vec<u64>,
    pub right_values: Vec<u64>,
}

impl std::str::FromStr for Input {
    type Err = aoc_libraries::aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let locations = parser!(lines(u64 "   " u64)).parse(content)?;
        let (left_values, right_values) = locations.into_iter().unzip();

        Ok(Self {
            left_values,
            right_values,
        })
    }
}

#[aoc_submission(
    sample_in = r"3   4
4   3
2   5
1   3
3   9
3   3",
    sample_out = 11
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    let mut left = input.left_values;
    let mut right = input.right_values;
    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum::<u64>()
}

#[aoc_submission(
    sample_in = r"3   4
4   3
2   5
1   3
3   9
3   3",
    sample_out = 31
)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    let right_counts = input
        .right_values
        .into_iter()
        .collect::<Counter<u64, u64>>();

    input
        .left_values
        .into_iter()
        .map(|value| u128::from(value) * u128::from(right_counts[&value]))
        .sum::<u128>()
}
