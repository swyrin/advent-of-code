use macros::{AocInput, aoc, part, sample};

#[derive(AocInput)]
struct Input {
    #[parse(string(any_char*))]
    content: String,
}
aoc!();

#[part]
#[sample(
    input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ",
    expected = "4277556"
)]
fn part_one(
    Input {
        content,
    }: &Input,
) -> impl std::fmt::Display {
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

    total
}

#[part]
#[sample(
    input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ",
    expected = "3263827"
)]
fn part_two(
    Input {
        content,
    }: &Input,
) -> impl std::fmt::Display {
    let mut total = 0u128;
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let row_count = grid.len();
    let column_count = grid.first().map_or(0, |row| row.len());
    let mut numbers: Vec<u32> = vec![];

    for column in (0..column_count).rev() {
        let mut parsed_number = 0;

        #[allow(clippy::needless_range_loop, reason = "look under.")]
        for row in 0..row_count - 1 {
            let character = grid[row][column];

            if character != ' ' {
                parsed_number = parsed_number * 10 + character.to_digit(10).expect("NaN");
            }
        }

        if parsed_number == 0 {
            numbers.clear();
            continue;
        }

        numbers.push(parsed_number);

        let operand = grid[row_count - 1][column];

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

    total
}
