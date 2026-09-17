use aoc_parse::{parser, prelude::*};

fn main() {
    use std::io::Read;

    let mut input_content = std::fs::File::open("input.txt").expect("No input.txt file");
    let mut buffer = String::new();

    input_content.read_to_string(&mut buffer).unwrap();
    let input: Input = buffer.as_str().parse().unwrap();

    let ans1 = part_one(&input);
    let ans2 = part_two(&input);

    if let Ok(ans1) = ans1 {
        println!("Result of part 1: {ans1}")
    } else {
        println!("Part 1 fails.")
    }

    if let Ok(ans2) = ans2 {
        println!("Result of part 2: {ans2}")
    } else {
        println!("Part 2 fails.")
    }
}

#[derive(Clone, Copy)]
struct Machine {
    button_a: (i128, i128),
    button_b: (i128, i128),
    prize: (i128, i128),
}

struct Input {
    machines: Vec<Machine>,
}

impl std::str::FromStr for Input {
    type Err = aoc_parse::ParseError;

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

fn total_cost(input: &Input, offset: i128, max_presses: Option<i128>) -> u128 {
    input
        .machines
        .iter()
        .copied()
        .filter_map(|machine| token_cost(machine, offset, max_presses))
        .sum()
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(total_cost(input, 0, Some(100)))
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(total_cost(input, 10_000_000_000_000, None))
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = { r"Button A: X+94, Y+34
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
Prize: X=18641, Y=10279" }, expected = { "480" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"Button A: X+94, Y+34
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
Prize: X=18641, Y=10279" }, expected = { "875318608908" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
