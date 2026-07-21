use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_macros::aoc_submission;

pub type Report = Vec<u32>;

pub struct Input {
    pub reports: Vec<Report>,
}

impl std::str::FromStr for Input {
    type Err = aoc_libraries::aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let reports = parser!(lines(repeat_sep(u32, " "))).parse(content)?;

        Ok(Self { reports })
    }
}

fn is_safe(report: &[u32]) -> bool {
    let increasing = report
        .windows(2)
        .all(|pair| pair[0] < pair[1] && (1..=3).contains(&pair[0].abs_diff(pair[1])));
    let decreasing = report
        .windows(2)
        .all(|pair| pair[0] > pair[1] && (1..=3).contains(&pair[0].abs_diff(pair[1])));

    increasing || decreasing
}

#[aoc_submission(
    sample_in = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    sample_out = 2
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    input
        .reports
        .iter()
        .filter(|report| is_safe(report))
        .count()
}

#[aoc_submission(
    sample_in = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    sample_out = 4
)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    input
        .reports
        .iter()
        .filter(|report| {
            is_safe(report)
                || (0..report.len()).any(|removed| {
                    let candidate = report
                        .iter()
                        .enumerate()
                        .filter_map(|(index, value)| (index != removed).then_some(*value))
                        .collect::<Vec<_>>();
                    is_safe(&candidate)
                })
        })
        .count()
}
