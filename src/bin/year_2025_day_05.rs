use macros::{AocInput, aoc, part, sample};
use range_set_blaze::RangeSetBlaze;

#[derive(AocInput)]
struct Input {
    #[parse(rs:section(lines(a:i64 "-" b:i64 => a..=b)) => RangeSetBlaze::from_iter(rs))]
    pub(crate) ranges: RangeSetBlaze<i64>,
    #[parse(section(lines(i64)))]
    pub(crate) numbers: Vec<i64>,
}
aoc!();

#[part]
#[sample(
    input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    expected = "3"
)]
fn part_one(
    Input {
        ranges,
        numbers,
    }: &Input,
) -> impl std::fmt::Display {
    numbers.iter().filter(|&number| ranges.contains(*number)).count()
}

#[part]
#[sample(
    input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    expected = "14"
)]
fn part_two(
    Input {
        ranges, ..
    }: &Input,
) -> impl std::fmt::Display {
    ranges.len()
}
