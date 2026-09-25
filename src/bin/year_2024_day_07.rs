use std::collections::HashSet;

use macros::{AocInput, aoc, part, sample};

#[derive(AocInput)]
struct Input {
    #[parse(lines(u64 ": " repeat_sep(u64, " ")))]
    pub(crate) equations: Vec<(u64, Vec<u64>)>,
}

fn concatenate(left: u64, right: u64) -> Option<u64> {
    let mut factor = 10_u64;
    let mut remaining = right;
    while remaining >= 10 {
        factor = factor.checked_mul(10)?;
        remaining /= 10;
    }
    left.checked_mul(factor)?.checked_add(right)
}

fn can_solve(target: u64, values: &[u64], allow_concatenation: bool) -> bool {
    fn search(
        target: u64,
        values: &[u64],
        index: usize,
        current: u64,
        allow_concatenation: bool,
        failed: &mut HashSet<(usize, u64)>,
    ) -> bool {
        if index == values.len() {
            return current == target;
        }
        if failed.contains(&(index, current)) {
            return false;
        }

        let next = values[index];
        let solved = current.checked_add(next).is_some_and(|value| {
            search(target, values, index + 1, value, allow_concatenation, failed)
        }) || current.checked_mul(next).is_some_and(|value| {
            search(target, values, index + 1, value, allow_concatenation, failed)
        }) || (allow_concatenation
            && concatenate(current, next).is_some_and(|value| {
                search(target, values, index + 1, value, allow_concatenation, failed)
            }));

        if !solved {
            failed.insert((index, current));
        }
        solved
    }

    let Some((&first, _)) = values.split_first() else {
        return false;
    };
    search(target, values, 1, first, allow_concatenation, &mut HashSet::new())
}

fn calibration_result(input: &Input, allow_concatenation: bool) -> u128 {
    input
        .equations
        .iter()
        .filter(|(target, values)| can_solve(*target, values, allow_concatenation))
        .map(|(target, _)| u128::from(*target))
        .sum()
}
aoc!();

#[part]
#[sample(
    input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
    expected = "3749"
)]
fn part_one(
    input @ Input {
        ..
    }: &Input,
) -> impl std::fmt::Display {
    calibration_result(input, false)
}

#[part]
#[sample(
    input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
    expected = "11387"
)]
fn part_two(
    input @ Input {
        ..
    }: &Input,
) -> impl std::fmt::Display {
    calibration_result(input, true)
}
