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
    sample_in = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ",
    sample_out = 4277556
)]
pub fn part_1(input: Input) -> UmiAteTheOutput {
    let content = input.content;
    let mut total = 0u128;

    let mut lines: Vec<Vec<&str>> = vec![];

    for line in content.lines() {
        let filtered_empty = line.split(' ').filter(|x| !x.is_empty()).collect();
        lines.push(filtered_empty);
    }

    let line_count = lines.len();
    let operands = lines.last().expect("No last element?");
    let numbers: Vec<&Vec<&str>> = lines.iter().take(line_count - 1).collect();
    let width = lines[0].len();
    let height = line_count - 1;

    for column in 0..width {
        let is_multiplication = operands[column] == "*";
        let mut value = if is_multiplication { 1 } else { 0 };

        for row in numbers.iter().take(height) {
            let number = row[column].parse::<i32>().expect("NaN");

            if is_multiplication {
                value *= number;
            } else {
                value += number;
            }
        }

        total += value as u128;
    }

    UmiAteTheOutput::from_number(total)
}

#[aoc_submission(
    input_type = crate::Input,
    sample_in = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ",
    sample_out = 3263827
)]
pub fn part_2(input: Input) -> UmiAteTheOutput {
    let content = input.content;
    let mut total = 0u128;
    let lines: Vec<&str> = content.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut numbers: Vec<u32> = vec![];

    for column in (0..width).rev() {
        let number_count = height - 1;
        let mut parsed_number = 0;

        for line in lines.iter().take(number_count) {
            let character = line.chars().nth(column).expect("Huh?");

            if character != ' ' {
                parsed_number = parsed_number * 10 + character.to_digit(10).expect("NaN");
            }
        }

        if parsed_number == 0 {
            numbers.clear();
            continue;
        }

        numbers.push(parsed_number);

        let operand = lines[height - 1]
            .chars()
            .nth(column)
            .expect("Did you turn off whitespace trim?");

        if operand != ' ' {
            let is_multiplication = operand == '*';
            let mut value = if is_multiplication { 1 } else { 0 };

            for number in &numbers {
                if is_multiplication {
                    value *= *number;
                } else {
                    value += *number;
                }
            }

            total += value as u128;
        }
    }

    UmiAteTheOutput::from_number(total)
}
