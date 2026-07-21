use aoc_libraries::pathfinding::matrix::Matrix;
use aoc_libraries::pathfinding::prelude::{bfs_reach, count_paths};
use aoc_macros::aoc_submission;

pub type Position = (usize, usize);

pub struct Input {
    pub grid: Matrix<u8>,
}

impl std::str::FromStr for Input {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let rows = content
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|height| {
                        height
                            .to_digit(10)
                            .map(|height| height as u8)
                            .ok_or_else(|| format!("invalid trail height: {height}"))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let grid = Matrix::from_rows(rows)
            .map_err(|error| format!("invalid topographic map: {error:?}"))?;

        Ok(Self { grid })
    }
}

impl Input {
    fn next_steps(&self, position: Position) -> Vec<Position> {
        let height = self.grid[position];
        self.grid
            .neighbours(position, false)
            .filter(|&next| self.grid[next] == height + 1)
            .collect()
    }

    fn trailheads(&self) -> impl Iterator<Item = Position> + '_ {
        self.grid
            .keys()
            .filter(|&position| self.grid[position] == 0)
    }
}

#[aoc_submission(
    sample_in = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    sample_out = 36
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    input
        .trailheads()
        .map(|trailhead| {
            bfs_reach(trailhead, |&position| input.next_steps(position))
                .filter(|&position| input.grid[position] == 9)
                .count()
        })
        .sum::<usize>()
}

#[aoc_submission(
    sample_in = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    sample_out = 81
)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    input
        .trailheads()
        .map(|trailhead| {
            count_paths(
                trailhead,
                |&position| input.next_steps(position),
                |&position| input.grid[position] == 9,
            )
        })
        .sum::<usize>()
}
