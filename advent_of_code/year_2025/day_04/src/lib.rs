use aoc_libraries::core::aoc_input::AocInput;
use aoc_libraries::core::aoc_output::AocOutput;
use aoc_libraries::pathfinding::matrix::Matrix;
use aoc_macros::aoc_submission;

pub struct Input {
    pub grid: Matrix<char>,
}

impl AocInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let grid = Matrix::from_rows(content.lines().map(str::chars)).unwrap();

        Self { grid }
    }
}

#[aoc_submission(
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
pub fn part_1(input: Input) -> AocOutput {
    let grid = input.grid;
    let paper_rolls = grid
        .keys()
        .filter(|&position| {
            grid[position] == '@'
                && grid
                    .neighbours(position, true)
                    .filter(|&neighbor| grid[neighbor] == '@')
                    .count()
                    < 4
        })
        .count();

    AocOutput::from_number(paper_rolls)
}

#[aoc_submission(
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
pub fn part_2(input: Input) -> AocOutput {
    let mut grid = input.grid;
    let mut destroy_count = 0;

    loop {
        let removable: Vec<_> = grid
            .keys()
            .filter(|&position| {
                grid[position] == '@'
                    && grid
                        .neighbours(position, true)
                        .filter(|&neighbor| grid[neighbor] == '@')
                        .count()
                        < 4
            })
            .collect();

        if removable.is_empty() {
            break;
        }

        destroy_count += removable.len();
        for position in removable {
            grid[position] = '.';
        }
    }

    AocOutput::from_number(destroy_count)
}
