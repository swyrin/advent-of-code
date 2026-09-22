use std::collections::{HashSet, VecDeque};

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
use macros::{AocInput, aoc};

#[derive(Debug, Clone)]
struct Machine {
    target: Vec<bool>,
    toggles: Vec<Vec<usize>>,
    jolts: Vec<u32>,
}

#[derive(AocInput)]
struct Input {
    #[parse(
        light = {
            "." => false,
            "#" => true,
        },
        button = "(" repeat_sep(usize, ",") ")",
        lines(
            "[" target:light+ "] "
            toggles:repeat_sep(button, " ")
            " {" jolts:repeat_sep(u32, ",") "}"
                => Machine { target, toggles, jolts }
        )
    )]
    pub(crate) machines: Vec<Machine>,
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

aoc! {
    #[sample(
        input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        expected = "7"
    )]
    fn part_one(Input { machines }: &Input) -> impl std::fmt::Display {
        machines.iter().map(fewest_presses).sum::<usize>()
    }

    #[sample(
        input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        expected = "33"
    )]
    fn part_two(Input { machines }: &Input) -> impl std::fmt::Display {
        let mut total = 0_u128;

        for machine in machines {
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

        total
    }
}
