use std::collections::{HashSet, VecDeque};

use aoc_parse::parser;
use aoc_parse::prelude::*;
use good_lp::{
    Expression,
    IntoAffineExpression,
    Solution,
    SolverModel,
    Variable,
    microlp,
    variable,
    variables,
};

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

struct Machine {
    target: Vec<bool>,
    toggles: Vec<Vec<usize>>,
    jolts: Vec<u32>,
}

struct Input {
    machines: Vec<Machine>,
}

impl std::str::FromStr for Input {
    type Err = aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let light = parser!({
            "." => false,
            "#" => true,
        });
        let button = parser!("(" repeat_sep(usize, ",") ")");
        let machines = parser!(lines(
            "[" target:light+ "] "
            toggles:repeat_sep(button, " ")
            " {" jolts:repeat_sep(u32, ",") "}"
                => Machine { target, toggles, jolts }
        ))
        .parse(content)?;

        Ok(Self {
            machines,
        })
    }
}

fn fewest_presses(machine: &Machine) -> usize {
    let start = vec![false; machine.target.len()];
    let mut visited = HashSet::new();
    visited.insert(start.clone());

    let mut queue = VecDeque::from([(start, 0_usize)]);

    while let Some((configuration, distance)) = queue.pop_front() {
        if configuration == machine.target {
            return distance;
        }

        for toggle in &machine.toggles {
            let mut next = configuration.clone();
            for &index in toggle {
                next[index] = !next[index];
            }

            if visited.insert(next.clone()) {
                queue.push_back((next, distance + 1));
            }
        }
    }

    panic!("the target configuration should be reachable");
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input.machines.iter().map(fewest_presses).sum::<usize>())
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut total = 0_u128;

    for machine in &input.machines {
        let mut variables = variables!();
        let presses: Vec<Variable> = (0..machine.toggles.len())
            .map(|_| variables.add(variable().min(0).integer()))
            .collect();

        let mut optimization = microlp(variables.minimise(presses.iter().sum::<Expression>()));
        let mut expressions = vec![0.into_expression(); machine.jolts.len()];

        for (press, toggled_outputs) in presses.iter().zip(&machine.toggles) {
            for &output in toggled_outputs {
                expressions[output] += *press;
            }
        }

        for (expression, jolt) in expressions.into_iter().zip(&machine.jolts) {
            optimization.add_constraint(expression.eq(*jolt as f64));
        }

        let solution = optimization.solve().unwrap();
        let press_count = presses.iter().map(|&press| solution.value(press)).sum::<f64>();
        total += press_count.round() as u128;
    }

    Ok(total)
}

#[cfg(test)]
mod test {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(input = { r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}" }, expected = { "7" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}" }, expected = { "33" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
