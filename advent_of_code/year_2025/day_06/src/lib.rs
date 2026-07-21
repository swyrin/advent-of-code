use aoc_libraries::core::aoc_input::AocInput;
use aoc_libraries::core::aoc_output::AocOutput;
use aoc_libraries::pathfinding::matrix::Matrix;
use aoc_macros::aoc_submission;

pub struct Input {
    pub content: String,
}

impl AocInput for Input {
    fn from_raw_string(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}

#[aoc_submission(
    sample_in = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ",
    sample_out = 4277556
)]
pub fn part_1(input: Input) -> AocOutput {
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

    AocOutput::from_number(total)
}

#[aoc_submission(
    sample_in = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ",
    sample_out = 3263827
)]
pub fn part_2(input: Input) -> AocOutput {
    let mut total = 0u128;
    let grid = Matrix::from_rows(input.content.lines().map(str::chars)).unwrap();
    let mut numbers: Vec<u32> = vec![];

    for column in (0..grid.columns).rev() {
        let mut parsed_number = 0;

        for row in 0..grid.rows - 1 {
            let character = grid[(row, column)];

            if character != ' ' {
                parsed_number = parsed_number * 10 + character.to_digit(10).expect("NaN");
            }
        }

        if parsed_number == 0 {
            numbers.clear();
            continue;
        }

        numbers.push(parsed_number);

        let operand = grid[(grid.rows - 1, column)];

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

    AocOutput::from_number(total)
}
