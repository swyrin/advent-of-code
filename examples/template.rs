use macros::{AocInput, aoc};

#[derive(AocInput)]
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
    fn part_one(Input { .. }: &Input) -> impl std::fmt::Display {
        69
    }

    #[sample(
        input = "420",
        expected = "420"
    )]
    fn part_two(Input { .. }: &Input) -> impl std::fmt::Display {
        420
    }
}
