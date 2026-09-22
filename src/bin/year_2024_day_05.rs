use std::collections::{HashMap, HashSet};

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

struct Input {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

impl std::str::FromStr for Input {
    type Err = aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let (rules, updates) = parser!(
            section(lines(u32 "|" u32))
            section(lines(repeat_sep(u32, ",")))
        )
        .parse(content)?;

        Ok(Self {
            rules,
            updates,
        })
    }
}

fn is_ordered(update: &[u32], rules: &[(u32, u32)]) -> bool {
    let positions =
        update.iter().enumerate().map(|(index, page)| (*page, index)).collect::<HashMap<_, _>>();

    rules.iter().all(|(before, after)| {
        let Some(before_index) = positions.get(before) else {
            return true;
        };
        let Some(after_index) = positions.get(after) else {
            return true;
        };
        before_index < after_index
    })
}

fn reorder(update: &[u32], rules: &[(u32, u32)]) -> Vec<u32> {
    let pages = update.iter().copied().collect::<HashSet<_>>();
    let mut indegree =
        update.iter().copied().map(|page| (page, 0_usize)).collect::<HashMap<_, _>>();
    let mut outgoing = HashMap::<u32, Vec<u32>>::new();
    let mut edges = HashSet::new();

    for &(before, after) in rules {
        if pages.contains(&before) && pages.contains(&after) && edges.insert((before, after)) {
            outgoing.entry(before).or_default().push(after);
            *indegree.get_mut(&after).expect("page must exist") += 1;
        }
    }

    let mut ordered = Vec::with_capacity(update.len());
    let mut emitted = HashSet::new();

    while ordered.len() < update.len() {
        let Some(next) =
            update.iter().copied().find(|page| !emitted.contains(page) && indegree[page] == 0)
        else {
            return update.to_vec();
        };

        emitted.insert(next);
        ordered.push(next);
        for after in outgoing.get(&next).into_iter().flatten() {
            *indegree.get_mut(after).expect("page must exist") -= 1;
        }
    }

    ordered
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .updates
        .iter()
        .filter(|update| is_ordered(update, &input.rules))
        .map(|update| u64::from(update[update.len() / 2]))
        .sum::<u64>())
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .updates
        .iter()
        .filter(|update| !is_ordered(update, &input.rules))
        .map(|update| reorder(update, &input.rules))
        .map(|update| u64::from(update[update.len() / 2]))
        .sum::<u64>())
}

#[cfg(test)]
mod test {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(input = { r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47" }, expected = { "143" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47" }, expected = { "123" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
