use aoc_parse::Parser;
use aoc_parse::prelude::*;
use macros::{AocInput, aoc};

#[derive(Debug, Clone, AocInput)]
struct Input {
    #[allow(unused)]
    #[parse(line(u32))]
    pub(crate) number: u32,
}

aoc! {
    #[sample(
        input = "69",
        expected = "69"
    )]
    fn part_one(_input: &Input) -> anyhow::Result<impl std::fmt::Display> {
        Ok(69)
    }

    #[sample(
        input = "420",
        expected = "420"
    )]
    fn part_two(_input: &Input) -> anyhow::Result<impl std::fmt::Display> {
        Ok(420)
    }
}
