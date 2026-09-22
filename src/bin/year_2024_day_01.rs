use counter::Counter;
use macros::{AocInput, aoc};

#[derive(AocInput)]
struct Input {
    #[parse(lines(u64 "   " u64))]
    pub(crate) pairs: Vec<(u64, u64)>,
}

aoc! {
    #[sample(
        input = "3   4
4   3
2   5
1   3
3   9
3   3",
        expected = "11"
    )]
    fn part_one(Input { pairs }: &Input) -> impl std::fmt::Display {
        let (mut left, mut right): (Vec<u64>, Vec<u64>) =
            pairs.iter().copied().unzip();
        left.sort_unstable();
        right.sort_unstable();

        left.into_iter().zip(right).map(|(left, right)| left.abs_diff(right)).sum::<u64>()
    }

    #[sample(
        input = "3   4
4   3
2   5
1   3
3   9
3   3",
        expected = "31"
    )]
    fn part_two(Input { pairs }: &Input) -> impl std::fmt::Display {
        let (left_values, right_values): (Vec<u64>, Vec<u64>) =
            pairs.iter().copied().unzip();
        let right_counts = right_values.iter().copied().collect::<Counter<u64, u64>>();

        left_values
            .iter()
            .map(|&value| u128::from(value) * u128::from(right_counts[&value]))
            .sum::<u128>()
    }
}
