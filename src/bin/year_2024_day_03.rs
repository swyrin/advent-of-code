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
    memory: String,
}

impl std::str::FromStr for Input {
    type Err = std::convert::Infallible;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            memory: content.to_owned(),
        })
    }
}

fn parse_number(bytes: &[u8], cursor: &mut usize) -> Option<u128> {
    let start = *cursor;
    let mut value = 0_u128;

    while *cursor - start < 3
        && let Some(digit) = bytes.get(*cursor).filter(|digit| digit.is_ascii_digit())
    {
        value = value * 10 + u128::from(*digit - b'0');
        *cursor += 1;
    }

    if bytes.get(*cursor).is_some_and(u8::is_ascii_digit) {
        return None;
    }

    (*cursor > start).then_some(value)
}

fn parse_mul(bytes: &[u8], start: usize) -> Option<(u128, usize)> {
    if !bytes.get(start..)?.starts_with(b"mul(") {
        return None;
    }

    let mut cursor = start + 4;
    let left = parse_number(bytes, &mut cursor)?;
    if bytes.get(cursor) != Some(&b',') {
        return None;
    }
    cursor += 1;

    let right = parse_number(bytes, &mut cursor)?;
    if bytes.get(cursor) != Some(&b')') {
        return None;
    }

    Some((left * right, cursor + 1))
}

fn sum_multiplications(memory: &str, honor_conditionals: bool) -> u128 {
    let bytes = memory.as_bytes();
    let mut enabled = true;
    let mut total = 0;
    let mut cursor = 0;

    while cursor < bytes.len() {
        let remaining = &bytes[cursor..];
        if honor_conditionals && remaining.starts_with(b"do()") {
            enabled = true;
            cursor += 4;
        } else if honor_conditionals && remaining.starts_with(b"don't()") {
            enabled = false;
            cursor += 7;
        } else if let Some((product, next)) = parse_mul(bytes, cursor) {
            if enabled {
                total += product;
            }
            cursor = next;
        } else {
            cursor += 1;
        }
    }

    total
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(sum_multiplications(&input.memory, false))
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(sum_multiplications(&input.memory, true))
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = { r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))" }, expected = { "161" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))" }, expected = { "48" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
