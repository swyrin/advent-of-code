use std::collections::HashMap;

use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_macros::aoc_submission;

pub struct Input {
    pub stones: Vec<u128>,
}

impl std::str::FromStr for Input {
    type Err = aoc_libraries::aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let stones = parser!(repeat_sep(u128, " ")).parse(content.trim())?;

        Ok(Self { stones })
    }
}

fn digit_count(value: u128) -> u32 {
    if value == 0 { 1 } else { value.ilog10() + 1 }
}

fn blink(input: Input, times: usize) -> u128 {
    let mut stones = HashMap::<u128, u128>::new();
    for stone in input.stones {
        *stones.entry(stone).or_default() += 1;
    }

    for _ in 0..times {
        let mut next = HashMap::<u128, u128>::new();

        for (stone, count) in stones {
            if stone == 0 {
                *next.entry(1).or_default() += count;
            } else {
                let digits = digit_count(stone);
                if digits.is_multiple_of(2) {
                    let divisor = 10_u128.pow(digits / 2);
                    *next.entry(stone / divisor).or_default() += count;
                    *next.entry(stone % divisor).or_default() += count;
                } else {
                    *next.entry(stone * 2024).or_default() += count;
                }
            }
        }

        stones = next;
    }

    stones.values().sum()
}

#[aoc_submission(sample_in = "125 17", sample_out = 55312)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    blink(input, 25)
}

#[aoc_submission(sample_in = "125 17", sample_out = 65601038650482u128)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    blink(input, 75)
}
