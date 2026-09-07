use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};

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
    stones: Vec<u128>,
}

impl std::str::FromStr for Input {
    type Err = aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let stones = parser!(repeat_sep(u128, " ")).parse(content.trim())?;

        Ok(Self { stones })
    }
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

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(blink(input, 25))
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(blink(input, 75))
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = { "125 17" }, expected = { "55312" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { "125 17" }, expected = { "65601038650482" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
