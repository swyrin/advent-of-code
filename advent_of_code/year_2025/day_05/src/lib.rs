use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_libraries::core::aoc_input::AocInput;
use aoc_libraries::range_set_blaze::RangeSetBlaze;
use aoc_macros::aoc_submission;

pub struct Input {
    pub ranges: RangeSetBlaze<i64>,
    pub numbers: Vec<i64>,
}

impl AocInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let (ranges, numbers) = parser!(section(lines(i64 "-" i64)) section(lines(i64)))
            .parse(content)
            .unwrap();
        let ranges = RangeSetBlaze::from_iter(ranges.into_iter().map(|(start, end)| start..=end));

        Self { ranges, numbers }
    }
}

#[aoc_submission(
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
pub fn part_1(input: Input) -> impl std::fmt::Display {
    input
        .numbers
        .into_iter()
        .filter(|number| input.ranges.contains(*number))
        .count()
}

#[aoc_submission(
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
pub fn part_2(input: Input) -> impl std::fmt::Display {
    input.ranges.len()
}
