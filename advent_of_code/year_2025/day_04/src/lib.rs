use aoc_macros::aoc_submission;
use aoc_utils::traits::generalised_output::UmiAteTheOutput;
use aoc_utils::traits::parsable_input::ParsableInput;
use std::collections::HashSet;

pub struct Input {
    pub grid: Vec<Vec<char>>,
}

impl ParsableInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let grid = content.lines().map(|line| line.chars().collect()).collect();

        Self { grid }
    }
}

#[aoc_submission(
    input_type = crate::Input,
    sample_in = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    sample_out = 13
)]
pub fn part_1(input: Input) -> UmiAteTheOutput {
    let grid = input.grid;
    let mut paper_rolls = 0;

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let is_a_paper_roll = |r: i32, c: i32| {
        0 <= r && r < rows && 0 <= c && c < cols && grid[r as usize][c as usize] == '@'
    };

    for i in 0..rows {
        for j in 0..cols {
            let mut count = 0;

            if grid[i as usize][j as usize] == '.' {
                continue;
            }

            // 12h
            if is_a_paper_roll(i - 1, j) {
                count += 1;
            }

            // 1h30
            if is_a_paper_roll(i - 1, j + 1) {
                count += 1;
            }

            // 3h
            if is_a_paper_roll(i, j + 1) {
                count += 1;
            }

            // 4h30
            if is_a_paper_roll(i + 1, j + 1) {
                count += 1;
            }

            // 6h
            if is_a_paper_roll(i + 1, j) {
                count += 1;
            }

            // 7h30
            if is_a_paper_roll(i + 1, j - 1) {
                count += 1;
            }

            // 9h
            if is_a_paper_roll(i, j - 1) {
                count += 1;
            }

            // 10h30
            if is_a_paper_roll(i - 1, j - 1) {
                count += 1;
            }

            if count < 4 {
                paper_rolls += 1
            }
        }
    }

    UmiAteTheOutput::from_number(paper_rolls)
}

#[aoc_submission(
    input_type = crate::Input,
    sample_in = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    sample_out = 43
)]
pub fn part_2(input: Input) -> UmiAteTheOutput {
    let grid = input.grid;
    let mut ignore_list: HashSet<(i32, i32)> = HashSet::new();
    let mut destroy_count = 0;

    loop {
        let mut paper_rolls = 0;

        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;

        let is_a_paper_roll = |v: &Vec<Vec<char>>, i: &HashSet<(i32, i32)>, r: i32, c: i32| {
            0 <= r
                && r < rows
                && 0 <= c
                && c < cols
                && !i.contains(&(r, c))
                && v[r as usize][c as usize] == '@'
        };

        for i in 0..rows {
            for j in 0..cols {
                let mut count = 0;

                if ignore_list.contains(&(i, j)) {
                    continue;
                }

                if grid[i as usize][j as usize] == '.' {
                    ignore_list.insert((i, j));
                    continue;
                }

                // 12h
                if is_a_paper_roll(&grid, &ignore_list, i - 1, j) {
                    count += 1;
                }

                // 1h30
                if is_a_paper_roll(&grid, &ignore_list, i - 1, j + 1) {
                    count += 1;
                }

                // 3h
                if is_a_paper_roll(&grid, &ignore_list, i, j + 1) {
                    count += 1;
                }

                // 4h30
                if is_a_paper_roll(&grid, &ignore_list, i + 1, j + 1) {
                    count += 1;
                }

                // 6h
                if is_a_paper_roll(&grid, &ignore_list, i + 1, j) {
                    count += 1;
                }

                // 7h30
                if is_a_paper_roll(&grid, &ignore_list, i + 1, j - 1) {
                    count += 1;
                }

                // 9h
                if is_a_paper_roll(&grid, &ignore_list, i, j - 1) {
                    count += 1;
                }

                // 10h30
                if is_a_paper_roll(&grid, &ignore_list, i - 1, j - 1) {
                    count += 1;
                }

                if count < 4 {
                    paper_rolls += 1;
                    ignore_list.insert((i, j));
                }
            }
        }

        if paper_rolls == 0 {
            break;
        } else {
            destroy_count += paper_rolls;
        }
    }

    UmiAteTheOutput::from_number(destroy_count)
}
