use aoc_macros::aoc_submission;

pub struct Input {
    pub memory: String,
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

#[aoc_submission(
    sample_in = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    sample_out = 161
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    sum_multiplications(&input.memory, false)
}

#[aoc_submission(
    sample_in = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    sample_out = 48
)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    sum_multiplications(&input.memory, true)
}
