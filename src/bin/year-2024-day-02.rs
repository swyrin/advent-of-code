use aoc_parse::parser;
use aoc_parse::prelude::*;

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

type Report = Vec<u32>;

struct Input {
    reports: Vec<Report>,
}

impl std::str::FromStr for Input {
    type Err = aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let reports = parser!(lines(repeat_sep(u32, " "))).parse(content)?;

        Ok(Self {
            reports,
        })
    }
}

fn is_safe(report: &[u32]) -> bool {
    let increasing = report
        .windows(2)
        .all(|pair| pair[0] < pair[1] && (1..=3).contains(&pair[0].abs_diff(pair[1])));
    let decreasing = report
        .windows(2)
        .all(|pair| pair[0] > pair[1] && (1..=3).contains(&pair[0].abs_diff(pair[1])));

    increasing || decreasing
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input.reports.iter().filter(|report| is_safe(report)).count())
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .reports
        .iter()
        .filter(|report| {
            is_safe(report)
                || (0..report.len()).any(|removed| {
                    let candidate = report
                        .iter()
                        .enumerate()
                        .filter_map(|(index, value)| (index != removed).then_some(*value))
                        .collect::<Vec<_>>();
                    is_safe(&candidate)
                })
        })
        .count())
}

#[cfg(test)]
mod test {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(input = { r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9" }, expected = { "2" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9" }, expected = { "4" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
