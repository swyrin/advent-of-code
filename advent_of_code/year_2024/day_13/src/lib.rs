use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_macros::aoc_submission;

#[derive(Clone, Copy)]
pub struct Machine {
    pub button_a: (i128, i128),
    pub button_b: (i128, i128),
    pub prize: (i128, i128),
}

pub struct Input {
    pub machines: Vec<Machine>,
}

impl std::str::FromStr for Input {
    type Err = aoc_libraries::aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let machines = parser!(sections(
            line("Button A: X+" i128 ", Y+" i128)
            line("Button B: X+" i128 ", Y+" i128)
            line("Prize: X=" i128 ", Y=" i128)
        ))
        .parse(content)?
        .into_iter()
        .map(|(button_a, button_b, prize)| Machine {
            button_a,
            button_b,
            prize,
        })
        .collect();

        Ok(Self { machines })
    }
}

fn token_cost(machine: Machine, offset: i128, max_presses: Option<i128>) -> Option<u128> {
    let (ax, ay) = machine.button_a;
    let (bx, by) = machine.button_b;
    let px = machine.prize.0 + offset;
    let py = machine.prize.1 + offset;
    let determinant = ax * by - ay * bx;

    if determinant == 0 {
        return None;
    }

    let a_numerator = px * by - py * bx;
    let b_numerator = ax * py - ay * px;
    if a_numerator % determinant != 0 || b_numerator % determinant != 0 {
        return None;
    }

    let a = a_numerator / determinant;
    let b = b_numerator / determinant;
    if a < 0 || b < 0 || max_presses.is_some_and(|maximum| a > maximum || b > maximum) {
        return None;
    }

    u128::try_from(3 * a + b).ok()
}

fn total_cost(input: Input, offset: i128, max_presses: Option<i128>) -> u128 {
    input
        .machines
        .into_iter()
        .filter_map(|machine| token_cost(machine, offset, max_presses))
        .sum()
}

#[aoc_submission(
    sample_in = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    sample_out = 480
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    total_cost(input, 0, Some(100))
}

#[aoc_submission(
    sample_in = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    sample_out = 875318608908u128
)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    total_cost(input, 10_000_000_000_000, None)
}
