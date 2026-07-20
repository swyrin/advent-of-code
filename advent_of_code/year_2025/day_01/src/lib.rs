use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_macros::aoc_submission;
use aoc_utils::traits::generalised_output::UmiAteTheOutput;
use aoc_utils::traits::parsable_input::ParsableInput;

pub struct Input {
    pub moves: Vec<(char, i32)>,
}

impl ParsableInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let moves = parser!(lines({
            "L" amount:i32 => ('L', amount),
            "R" amount:i32 => ('R', amount),
        }))
        .parse(content)
        .unwrap();

        Self { moves }
    }
}

#[aoc_submission(
    input_type = crate::Input,
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
fn part_1(input: Input) -> UmiAteTheOutput {
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

    UmiAteTheOutput::from_number(count)
}

#[aoc_submission(
    input_type = crate::Input,
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
fn part_2(input: Input) -> UmiAteTheOutput {
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

    UmiAteTheOutput::from_number(count)
}
