use macros::{AocInput, aoc, part, sample};

type Report = Vec<u32>;

#[derive(AocInput)]
struct Input {
    #[parse(lines(repeat_sep(u32, " ")))]
    pub(crate) reports: Vec<Report>,
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
aoc!();

#[part]
#[sample(
    input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    expected = "2"
)]
fn part_one(
    Input {
        reports,
    }: &Input,
) -> impl std::fmt::Display {
    reports.iter().filter(|report| is_safe(report)).count()
}

#[part]
#[sample(
    input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    expected = "4"
)]
fn part_two(
    Input {
        reports,
    }: &Input,
) -> impl std::fmt::Display {
    reports
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
        .count()
}
