use macros::{AocInput, aoc, part, sample};

#[derive(AocInput)]
struct Input {
    #[allow(unused)]
    #[parse(line(u32))]
    pub(crate) number: u32,
}

#[derive(AocInput)]
struct Input2 {
    #[allow(unused)]
    #[parse(line(u32))]
    pub(crate) number: u32,
}

aoc!();

#[part]
#[sample(input = "69", expected = "69")]
fn part_one(
    Input {
        ..
    }: &Input,
) -> impl std::fmt::Display {
    69
}

#[part]
#[sample(input = "420", expected = "420")]
fn part_two(
    Input2 {
        ..
    }: &Input2,
) -> impl std::fmt::Display {
    420
}
