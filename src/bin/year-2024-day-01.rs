use aoc_parse::parser;
use aoc_parse::prelude::*;
use counter::Counter;

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
    left_values: Vec<u64>,
    right_values: Vec<u64>,
}

impl std::str::FromStr for Input {
    type Err = aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let locations = parser!(lines(u64 "   " u64)).parse(content)?;
        let (left_values, right_values) = locations.into_iter().unzip();

        Ok(Self {
            left_values,
            right_values,
        })
    }
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut left = input.left_values.clone();
    let mut right = input.right_values.clone();
    left.sort_unstable();
    right.sort_unstable();

    Ok(left.into_iter().zip(right).map(|(left, right)| left.abs_diff(right)).sum::<u64>())
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let right_counts = input.right_values.iter().copied().collect::<Counter<u64, u64>>();

    Ok(input
        .left_values
        .iter()
        .map(|&value| u128::from(value) * u128::from(right_counts[&value]))
        .sum::<u128>())
}

#[cfg(test)]
mod test {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(input = { r"3   4
4   3
2   5
1   3
3   9
3   3" }, expected = { "11" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"3   4
4   3
2   5
1   3
3   9
3   3" }, expected = { "31" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
