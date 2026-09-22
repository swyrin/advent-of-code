use std::collections::{HashMap, HashSet};

use macros::{AocInput, aoc};

#[derive(AocInput)]
struct Input {
    #[parse(section(lines(u32 "|" u32)))]
    pub(crate) rules: Vec<(u32, u32)>,
    #[parse(section(lines(repeat_sep(u32, ","))))]
    pub(crate) updates: Vec<Vec<u32>>,
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

aoc! {
    #[sample(
        input = "47|53
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
97,13,75,29,47",
        expected = "143"
    )]
    fn part_one(Input { rules, updates }: &Input) -> impl std::fmt::Display {
        updates
            .iter()
            .filter(|update| is_ordered(update, rules))
            .map(|update| u64::from(update[update.len() / 2]))
            .sum::<u64>()
    }

    #[sample(
        input = "47|53
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
97,13,75,29,47",
        expected = "123"
    )]
    fn part_two(Input { rules, updates }: &Input) -> impl std::fmt::Display {
        updates
            .iter()
            .filter(|update| !is_ordered(update, rules))
            .map(|update| reorder(update, rules))
            .map(|update| u64::from(update[update.len() / 2]))
            .sum::<u64>()
    }
}
