use std::cmp;
use std::collections::HashSet;

use aoc_macros::aoc_submission;
use aoc_utils::traits::generalised_output::UmiAteTheOutput;
use aoc_utils::traits::parsable_input::ParsableInput;

pub struct Input {
    pub ranges: HashSet<(i64, i64)>,
    pub numbers: Vec<i64>,
}

impl ParsableInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let mut reading_actual_input = false;
        let mut ranges = HashSet::new();
        let mut numbers = vec![];

        for line in content.lines() {
            if line.is_empty() {
                reading_actual_input = true;
                continue;
            }

            if !reading_actual_input {
                let heads: Vec<&str> = line.split('-').collect();
                let head = heads[0].parse::<i64>().unwrap();
                let tail = heads[1].parse::<i64>().unwrap();
                ranges.insert((head, tail));
            } else {
                numbers.push(line.parse::<i64>().unwrap());
            }
        }

        Self { ranges, numbers }
    }
}

#[aoc_submission(
    input_type = crate::Input,
    sample_in = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    sample_out = 3
)]
pub fn part_1(input: Input) -> UmiAteTheOutput {
    let mut count = 0;

    for number in input.numbers {
        let mut has_match = false;

        for range in input.ranges.clone() {
            let a = range.0;
            let b = range.1;

            if a <= number && number <= b {
                has_match = true;
                break;
            }
        }

        if has_match {
            count += 1;
        }
    }

    UmiAteTheOutput::from_number(count)
}

#[aoc_submission(
    input_type = crate::Input,
    sample_in = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    sample_out = 14
)]
pub fn part_2(input: Input) -> UmiAteTheOutput {
    let mut ranges: Vec<Vec<i64>> = input.ranges.into_iter().map(|(a, b)| vec![a, b]).collect();

    // It's been years since I last seen a range combinator
    ranges.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut combined = vec![ranges[0].clone()];

    for current in ranges.iter().skip(1) {
        let current = current.clone();
        let j = combined.len() - 1;

        if combined[j][0] <= current[0] && current[0] <= combined[j][1] {
            combined[j][1] = cmp::max(current[1], combined[j][1]);
        } else {
            combined.push(current);
        }
    }

    let total = combined
        .into_iter()
        .map(|range| (range[1] - range[0]) + 1)
        .sum::<i64>();

    UmiAteTheOutput::from_number(total)
}
