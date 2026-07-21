use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_libraries::good_lp::{
    Expression, IntoAffineExpression, Solution, SolverModel, Variable, microlp, variable, variables,
};
use aoc_libraries::pathfinding::prelude::bfs;
use aoc_macros::aoc_submission;

pub struct Machine {
    pub target: Vec<bool>,
    pub toggles: Vec<Vec<usize>>,
    pub jolts: Vec<u32>,
}

pub struct Input {
    pub machines: Vec<Machine>,
}

impl std::str::FromStr for Input {
    type Err = aoc_libraries::aoc_parse::ParseError;

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

        Ok(Self { machines })
    }
}

#[aoc_submission(
    sample_in = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    sample_out = 7
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    let total = input
        .machines
        .into_iter()
        .map(|machine| {
            let start = vec![false; machine.target.len()];
            bfs(
                &start,
                |configuration| {
                    machine
                        .toggles
                        .iter()
                        .map(move |toggle| {
                            let mut next = configuration.clone();
                            for &index in toggle {
                                next[index] = !next[index];
                            }
                            next
                        })
                        .collect::<Vec<_>>()
                },
                |configuration| configuration == &machine.target,
            )
            .expect("the target configuration should be reachable")
            .len()
                - 1
        })
        .sum::<usize>();

    total
}

#[aoc_submission(
    sample_in = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    sample_out = 33
)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    let mut total = 0_u128;

    for machine in input.machines {
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

        for (expression, jolt) in expressions.into_iter().zip(machine.jolts) {
            optimization.add_constraint(expression.eq(jolt as f64));
        }

        let solution = optimization.solve().unwrap();
        let press_count = presses
            .iter()
            .map(|&press| solution.value(press))
            .sum::<f64>();
        total += press_count.round() as u128;
    }

    total
}
