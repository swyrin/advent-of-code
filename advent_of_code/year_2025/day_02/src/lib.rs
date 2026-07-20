use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_macros::aoc_submission;
use aoc_utils::traits::generalised_output::UmiAteTheOutput;
use aoc_utils::traits::parsable_input::ParsableInput;

pub struct Input {
    pub ranges: Vec<(i64, i64)>,
}

impl ParsableInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let ranges = parser!(repeat_sep(i64 "-" i64, ","))
            .parse(content.trim())
            .unwrap();

        Self { ranges }
    }
}

#[aoc_submission(
    input_type = crate::Input,
    sample_in = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
    sample_out = 1227775554
)]
fn part_1(input: Input) -> UmiAteTheOutput {
    let mut result = 0;

    for (head, tail) in input.ranges {
        for i in head..=tail {
            let x = i.to_string();
            let l = x.len();

            if l % 2 != 0 {
                continue;
            }

            let first_half = &x[..(l / 2)];
            let second_half = &x[(l / 2)..];

            if first_half == second_half {
                result += i;
            }
        }
    }

    UmiAteTheOutput::from_number(result)
}

#[aoc_submission(
    input_type = crate::Input,
    sample_in = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
    sample_out = 4174379265u128
)]
fn part_2(input: Input) -> UmiAteTheOutput {
    let mut result = 0;

    for (head, tail) in input.ranges {
        for i in head..=tail {
            let x = i.to_string();
            let l = x.len();
            let mut has_match = false;

            for len in 1..l {
                if l % len != 0 {
                    continue;
                }

                let part = &x[..len];

                let count = x
                    .as_bytes()
                    .chunks(len)
                    .filter(|&x| x == part.as_bytes())
                    .count();

                if count * len == l {
                    has_match = true;
                    break;
                }
            }

            if has_match {
                result += i;
            }
        }
    }

    UmiAteTheOutput::from_number(result)
}
