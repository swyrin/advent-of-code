use aoc_macros::aoc_submission;

pub struct Input {
    pub everything: usize,
}

impl std::str::FromStr for Input {
    type Err = aoc_libraries::aoc_parse::ParseError;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Self { everything: 42 })
    }
}

#[aoc_submission(sample_in = r"", sample_out = 42)]
pub fn part_1(_: Input) -> impl std::fmt::Display {
    42
}

#[aoc_submission(ignore = "This test is from template.")]
pub fn part_2(_: Input) -> impl std::fmt::Display {
    42
}
