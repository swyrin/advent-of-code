use aoc_parse::prelude::*;
use aoc_parse::{Parser, parser};

fn main() {
    use std::io::Read;

    let mut input_content = std::fs::File::open("input.txt").expect("No input.txt file");
    let mut buffer = String::new();

    input_content.read_to_string(&mut buffer).unwrap();
    let input: Input = buffer.as_str().parse().unwrap();

    if let Ok(ans1) = part_one(&input) {
        println!("Result of part 1: {ans1}")
    } else {
        println!("Part 1 fails.")
    }

    if let Ok(ans2) = part_two(&input) {
        println!("Result of part 2: {ans2}")
    } else {
        println!("Part 2 fails.")
    }
}

struct Input {
    #[allow(unused)]
    pub(crate) number: u32,
}

impl std::str::FromStr for Input {
    type Err = aoc_parse::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = parser!(line(u32));

        Ok(Self {
            number: p.parse(s)?,
        })
    }
}

#[forbid(unsafe_code)]
fn part_one(_input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(69)
}

#[forbid(unsafe_code)]
fn part_two(_input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(420)
}

#[cfg(test)]
mod test {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(input = { "69" }, expected = { "69" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { "420" }, expected = { "420" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
