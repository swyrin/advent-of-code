use std::collections::HashMap;

use macros::{aoc, part, sample};

struct Input {
    pub(crate) patterns: Vec<Vec<char>>,
}

impl std::str::FromStr for Input {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(|x| x.chars().collect::<Vec<char>>()).collect();

        Ok(Self {
            patterns: lines,
        })
    }
}

aoc!();

#[part]
#[sample(
    input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
    expected = "26397"
)]
fn part_one(
    Input {
        patterns,
    }: &Input,
) -> impl std::fmt::Display {
    let mut total = 0;

    let bracket_map: HashMap<char, char> =
        HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let bracket_score: HashMap<char, i32> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    for pattern in patterns {
        let mut stack_at_home: Vec<char> = vec![];
        let mut invalid: Option<char> = None;

        for ch in pattern {
            if let Some(matched_opening) = bracket_map.get(ch) {
                if stack_at_home.is_empty() {
                    invalid = Some(*ch);
                    break;
                }

                let top = stack_at_home.last().unwrap();

                if top != matched_opening {
                    invalid = Some(*ch);
                    break;
                } else {
                    stack_at_home.pop().unwrap();
                }
            } else {
                stack_at_home.push(*ch);
            }
        }

        if let Some(bracket) = invalid
            && let Some(score) = bracket_score.get(&bracket)
        {
            total += score;
        }
    }

    total
}

#[part]
#[sample(
    input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
    expected = "288957"
)]
fn part_two(
    Input {
        patterns,
    }: &Input,
) -> impl std::fmt::Display {
    let mut scores = vec![];

    let open_bracket_map: HashMap<char, char> =
        HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let close_bracket_map: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let bracket_score: HashMap<char, i64> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    'outer: for pattern in patterns {
        println!("pattern {:?}", pattern);

        let mut pattern_score = 0i64;
        let mut stack_at_home: Vec<char> = vec![];

        for ch in pattern {
            if let Some(matched_opening) = open_bracket_map.get(ch) {
                // no left-over opens
                if stack_at_home.is_empty() {
                    // println!("Choked in middle\n");
                    continue 'outer;
                }

                let top = stack_at_home.last().unwrap();

                // no matching top
                if top != matched_opening {
                    // println!("Stack top {top} does not matched expected {matched_opening}\n");
                    continue 'outer;
                }

                stack_at_home.pop().unwrap();
            } else {
                stack_at_home.push(*ch);
            }
        }

        if stack_at_home.is_empty() {
            continue 'outer;
        }

        stack_at_home.reverse();

        let closes = stack_at_home
            .iter()
            .map(|open| close_bracket_map.get(open).unwrap())
            .collect::<Vec<_>>();

        for close in closes {
            if let Some(score) = bracket_score.get(close) {
                pattern_score *= 5;
                pattern_score += score;
            } else {
                unreachable!()
            }
        }

        scores.push(pattern_score);
    }

    scores.sort();

    // There will always be an odd number of scores to consider.
    scores[scores.len() / 2]
}

fn test() {}
