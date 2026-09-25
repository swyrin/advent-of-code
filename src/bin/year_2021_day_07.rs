use macros::{AocInput, aoc, part, sample};

#[derive(AocInput)]
struct Input {
    #[parse(line(repeat_sep(u32, ",")))]
    pub(crate) numbers: Vec<u32>,
}

aoc!();

#[part]
#[sample(input = "16,1,2,0,4,2,7,1,2,14", expected = "37")]
fn part_one(
    Input {
        numbers,
    }: &Input,
) -> impl std::fmt::Display {
    let mut sorted = numbers.clone();
    sorted.sort();

    let middle = sorted.len() / 2;

    // https://en.wikipedia.org/wiki/Geometric_median#Properties
    let median = if sorted.len() % 2 == 0 {
        (sorted[middle] + sorted[middle - 1]) / 2
    } else {
        sorted[middle]
    };

    numbers.iter().map(|x| x.abs_diff(median)).sum::<u32>()
}

// #[part]
// #[sample(
//     input = "16,1,2,0,4,2,7,1,2,14",
//     expected = "168"
// )]
// fn part_two(Input { .. }: &Input) -> impl std::fmt::Display {
//     420
// }
