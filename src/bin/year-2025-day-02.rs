use aoc_parse::{parser, prelude::*};

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
    ranges: Vec<(i64, i64)>,
}

impl std::str::FromStr for Input {
    type Err = aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let ranges = parser!(repeat_sep(i64 "-" i64, ",")).parse(content.trim())?;

        Ok(Self { ranges })
    }
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut result = 0;

    for &(head, tail) in &input.ranges {
        for i in head..=tail {
            let x = i.to_string();
            let l = x.len();

            if l % 2 != 0 {
                continue;
            }

            let first_half = &x[..(l / 2)];
            let second_half = &x[(l / 2)..];

            if first_half == second_half {
                result += i;
            }
        }
    }

    Ok(result)
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut result = 0;

    for &(head, tail) in &input.ranges {
        for i in head..=tail {
            let x = i.to_string();
            let l = x.len();
            let mut has_match = false;

            for len in 1..l {
                if l % len != 0 {
                    continue;
                }

                let part = &x[..len];

                let count = x
                    .as_bytes()
                    .chunks(len)
                    .filter(|&x| x == part.as_bytes())
                    .count();

                if count * len == l {
                    has_match = true;
                    break;
                }
            }

            if has_match {
                result += i;
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = { r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124" }, expected = { "1227775554" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124" }, expected = { "4174379265" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
