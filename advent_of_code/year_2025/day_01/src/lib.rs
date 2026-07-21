use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_macros::aoc_submission;

pub struct Input {
    pub moves: Vec<(char, i32)>,
}

impl std::str::FromStr for Input {
    type Err = aoc_libraries::aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let moves = parser!(lines({
            "L" amount:i32 => ('L', amount),
            "R" amount:i32 => ('R', amount),
        }))
        .parse(content)?;

        Ok(Self { moves })
    }
}

#[aoc_submission(
    sample_in = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
    sample_out = 3
)]
fn part_1(input: Input) -> impl std::fmt::Display {
    let mut pos = 50;
    let mut count = 0;

    for (direction, amount) in input.moves {
        match direction {
            'L' => {
                pos = (pos - amount) % 100;
            }
            'R' => {
                pos = (pos + amount) % 100;
            }
            _ => {
                panic!("Not a valid direction.")
            }
        }

        if pos == 0 {
            count += 1;
        }
    }

    count
}

#[aoc_submission(
    sample_in = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
    sample_out = 6
)]
fn part_2(input: Input) -> impl std::fmt::Display {
    let mut pos: i64 = 50;
    let mut count: i64 = 0;

    for (direction, amount) in input.moves {
        let amount = amount as i64;
        count += amount / 100;
        let amount = amount % 100;

        match direction {
            'L' => {
                if pos != 0 && pos - amount <= 0 {
                    count += 1;
                }

                pos = (pos - amount).rem_euclid(100);
            }
            'R' => {
                if pos != 0 && pos + amount >= 100 {
                    count += 1;
                }

                pos = (pos + amount).rem_euclid(100);
            }
            _ => {
                panic!("Not a valid direction.")
            }
        }
    }

    count
}
