use std::collections::HashMap;

use macros::{AocInput, aoc, part, sample};

#[derive(AocInput)]
struct Input {
    #[parse(line(repeat_sep(u128, " ")))]
    stones: Vec<u128>,
}

fn digit_count(value: u128) -> u32 {
    if value == 0 { 1 } else { value.ilog10() + 1 }
}

fn blink(input: &Input, times: usize) -> u128 {
    let mut stones = HashMap::<u128, u128>::new();
    for &stone in &input.stones {
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
aoc!();

#[part]
#[sample(input = "125 17", expected = "55312")]
fn part_one(
    input @ Input {
        ..
    }: &Input,
) -> impl std::fmt::Display {
    blink(input, 25)
}

#[part]
#[sample(input = "125 17", expected = "65601038650482")]
fn part_two(
    input @ Input {
        ..
    }: &Input,
) -> impl std::fmt::Display {
    blink(input, 75)
}
