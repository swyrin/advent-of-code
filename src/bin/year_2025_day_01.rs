use aoc_parse::parser;
use aoc_parse::prelude::*;

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
    moves: Vec<(char, i32)>,
}

impl std::str::FromStr for Input {
    type Err = aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let moves = parser!(lines({
            "L" amount:i32 => ('L', amount),
            "R" amount:i32 => ('R', amount),
        }))
        .parse(content)?;

        Ok(Self {
            moves,
        })
    }
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut pos = 50;
    let mut count = 0;

    for &(direction, amount) in &input.moves {
        match direction {
            'L' => {
                pos = (pos - amount) % 100;
            },
            'R' => {
                pos = (pos + amount) % 100;
            },
            _ => {
                panic!("Not a valid direction.")
            },
        }

        if pos == 0 {
            count += 1;
        }
    }

    Ok(count)
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut pos: i64 = 50;
    let mut count: i64 = 0;

    for &(direction, amount) in &input.moves {
        let amount = amount as i64;
        count += amount / 100;
        let amount = amount % 100;

        match direction {
            'L' => {
                if pos != 0 && pos - amount <= 0 {
                    count += 1;
                }

                pos = (pos - amount).rem_euclid(100);
            },
            'R' => {
                if pos != 0 && pos + amount >= 100 {
                    count += 1;
                }

                pos = (pos + amount).rem_euclid(100);
            },
            _ => {
                panic!("Not a valid direction.")
            },
        }
    }

    Ok(count)
}

#[cfg(test)]
mod test {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(input = { r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82" }, expected = { "3" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82" }, expected = { "6" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
