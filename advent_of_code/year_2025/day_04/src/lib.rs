use aoc_libraries::pathfinding::matrix::Matrix;
use aoc_macros::aoc_submission;
use aoc_utils::traits::generalised_output::UmiAteTheOutput;
use aoc_utils::traits::parsable_input::ParsableInput;

pub struct Input {
    pub grid: Matrix<char>,
}

impl ParsableInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let grid = Matrix::from_rows(content.lines().map(str::chars)).unwrap();

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

    UmiAteTheOutput::from_number(destroy_count)
}
