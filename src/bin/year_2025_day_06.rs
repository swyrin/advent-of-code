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

struct Input {
    content: String,
}

impl std::str::FromStr for Input {
    type Err = std::convert::Infallible;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            content: content.to_string(),
        })
    }
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut total = 0u128;

    let mut lines: Vec<Vec<&str>> = vec![];

    for line in input.content.lines() {
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

    Ok(total)
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut total = 0u128;
    let grid: Vec<Vec<char>> = input
        .content
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let row_count = grid.len();
    let column_count = grid.first().map_or(0, |row| row.len());
    let mut numbers: Vec<u32> = vec![];

    for column in (0..column_count).rev() {
        let mut parsed_number = 0;

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

    Ok(total)
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = { "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  " }, expected = { "4277556" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  " }, expected = { "3263827" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
