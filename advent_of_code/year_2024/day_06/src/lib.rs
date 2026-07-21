use std::collections::HashSet;

use aoc_libraries::pathfinding::matrix::Matrix;
use aoc_macros::aoc_submission;

pub type Position = (usize, usize);

pub struct Input {
    pub grid: Matrix<u8>,
    pub start: Position,
}

impl std::str::FromStr for Input {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let grid = Matrix::from_rows(
            content
                .lines()
                .filter(|line| !line.is_empty())
                .map(str::bytes),
        )
        .map_err(|error| format!("invalid guard map: {error:?}"))?;
        let start = grid
            .keys()
            .find(|&position| grid[position] == b'^')
            .ok_or_else(|| "guard map has no starting position".to_string())?;

        Ok(Self { grid, start })
    }
}

impl Input {
    fn walk(&self, extra_obstacle: Option<Position>) -> (bool, HashSet<Position>) {
        const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let mut position = self.start;
        let mut direction = 0;
        let mut visited = HashSet::from([position]);
        let mut states = HashSet::from([(position, direction)]);

        loop {
            let delta = DIRECTIONS[direction];
            let Some(next) = position
                .0
                .checked_add_signed(delta.0)
                .zip(position.1.checked_add_signed(delta.1))
            else {
                return (false, visited);
            };

            let Some(cell) = self.grid.get(next) else {
                return (false, visited);
            };

            if *cell == b'#' || extra_obstacle == Some(next) {
                direction = (direction + 1) % DIRECTIONS.len();
            } else {
                position = next;
                visited.insert(position);
            }

            if !states.insert((position, direction)) {
                return (true, visited);
            }
        }
    }
}

#[aoc_submission(
    sample_in = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
    sample_out = 41
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    input.walk(None).1.len()
}

#[aoc_submission(
    sample_in = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
    sample_out = 6
)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    input
        .walk(None)
        .1
        .into_iter()
        .filter(|&position| position != input.start)
        .filter(|&position| input.walk(Some(position)).0)
        .count()
}
