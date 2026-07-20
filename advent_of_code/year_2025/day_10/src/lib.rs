use std::collections::{HashSet, VecDeque};

use aoc_libraries::good_lp::{
    Expression, IntoAffineExpression, Solution, SolverModel, Variable, microlp, variable, variables,
};
use aoc_macros::aoc_submission;
use aoc_utils::traits::generalised_output::UmiAteTheOutput;
use aoc_utils::traits::parsable_input::ParsableInput;

pub struct Input {
    pub content: String,
}

impl ParsableInput for Input {
    fn from_raw_string(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}

#[aoc_submission(
    input_type = crate::Input,
    sample_in = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    sample_out = 7
)]
pub fn part_1(input: Input) -> UmiAteTheOutput {
    let mut total = 0;

    for line in input.content.lines() {
        let components: Vec<&str> = line.split(' ').collect();
        let configuration = components[0];
        let buttons = components.iter().skip(1).take(components.len() - 2);
        let toggles: Vec<Vec<usize>> = buttons
            .map(|button| {
                button
                    .trim_matches(['(', ')'])
                    .split(',')
                    .map(|index| index.parse().unwrap())
                    .collect()
            })
            .collect();

        let target: Vec<bool> = configuration
            .chars()
            .skip(1)
            .take(configuration.len() - 2)
            .map(|character| character == '#')
            .collect();
        let start = vec![false; target.len()];

        if start == target {
            continue;
        }

        let mut queue = VecDeque::from([(start.clone(), 0)]);
        let mut seen_states = HashSet::from([start]);

        while let Some((configuration, count)) = queue.pop_front() {
            if configuration == target {
                total += count;
                break;
            }

            for toggle in &toggles {
                let mut next_configuration = configuration.clone();

                for &index in toggle {
                    next_configuration[index] = !next_configuration[index];
                }

                if seen_states.insert(next_configuration.clone()) {
                    queue.push_back((next_configuration, count + 1));
                }
            }
        }
    }

    UmiAteTheOutput::from_number(total)
}

#[aoc_submission(
    input_type = crate::Input,
    sample_in = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    sample_out = 33
)]
pub fn part_2(input: Input) -> UmiAteTheOutput {
    let mut total = 0_u128;

    for line in input.content.lines() {
        let components: Vec<&str> = line.split(' ').collect();
        let buttons = components.iter().skip(1).take(components.len() - 2);
        let jolts: Vec<u32> = components
            .last()
            .expect("There isn't any?")
            .trim_matches(['{', '}'])
            .split(',')
            .map(|jolt| jolt.parse().unwrap())
            .collect();
        let toggles: Vec<Vec<usize>> = buttons
            .map(|button| {
                button
                    .trim_matches(['(', ')'])
                    .split(',')
                    .map(|index| index.parse().unwrap())
                    .collect()
            })
            .collect();

        let mut variables = variables!();
        let presses: Vec<Variable> = (0..toggles.len())
            .map(|_| variables.add(variable().min(0).integer()))
            .collect();

        let mut optimization = microlp(variables.minimise(presses.iter().sum::<Expression>()));
        let mut expressions = vec![0.into_expression(); jolts.len()];

        for (press, toggled_outputs) in presses.iter().zip(&toggles) {
            for &output in toggled_outputs {
                expressions[output] += *press;
            }
        }

        for (expression, jolt) in expressions.into_iter().zip(jolts) {
            optimization.add_constraint(expression.eq(jolt as f64));
        }

        let solution = optimization.solve().unwrap();
        let press_count = presses
            .iter()
            .map(|&press| solution.value(press))
            .sum::<f64>();
        total += press_count.round() as u128;
    }

    UmiAteTheOutput::from_number(total)
}
