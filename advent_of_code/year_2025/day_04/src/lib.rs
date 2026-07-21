use aoc_libraries::pathfinding::matrix::Matrix;
use aoc_macros::aoc_submission;

pub struct Input {
    pub grid: Matrix<char>,
}

impl std::str::FromStr for Input {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let grid = Matrix::from_rows(content.lines().map(str::chars))
            .map_err(|error| format!("invalid grid: {error:?}"))?;

        Ok(Self { grid })
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
pub fn part_1(input: Input) -> impl std::fmt::Display {
    let grid = input.grid;
    grid.keys()
        .filter(|&position| {
            grid[position] == '@'
                && grid
                    .neighbours(position, true)
                    .filter(|&neighbor| grid[neighbor] == '@')
                    .count()
                    < 4
        })
        .count()
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
pub fn part_2(input: Input) -> impl std::fmt::Display {
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

    destroy_count
}
