use std::collections::HashSet;

use macros::{AocInput, aoc};

type Position = (usize, usize);

#[derive(AocInput)]
struct Input {
    #[parse(rows:lines(string(any_char+)) => to_grid(rows))]
    pub(crate) grid: Vec<Vec<u8>>,
}

fn to_grid(rows: Vec<String>) -> Vec<Vec<u8>> {
    rows.into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

impl Input {
    fn get_start(&self) -> Position {
        for (row, line) in self.grid.iter().enumerate() {
            for (column, &cell) in line.iter().enumerate() {
                if cell == b'^' {
                    return (row, column);
                }
            }
        }

        panic!("guard map has no starting position");
    }

    fn cell(&self, (row, column): Position) -> Option<u8> {
        self.grid.get(row).and_then(|line| line.get(column)).copied()
    }

    fn walk(&self, extra_obstacle: Option<Position>) -> (bool, HashSet<Position>) {
        const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let mut position = self.get_start();
        let mut direction = 0;
        let mut visited = HashSet::from([position]);
        let mut states = HashSet::from([(position, direction)]);

        loop {
            let delta = DIRECTIONS[direction];
            let Some(next) =
                position.0.checked_add_signed(delta.0).zip(position.1.checked_add_signed(delta.1))
            else {
                return (false, visited);
            };

            let Some(cell) = self.cell(next) else {
                return (false, visited);
            };

            if cell == b'#' || extra_obstacle == Some(next) {
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

aoc! {
    #[sample(
        input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        expected = "41"
    )]
    fn part_one(input @ Input { .. }: &Input) -> impl std::fmt::Display {
        input.walk(None).1.len()
    }

    #[sample(
        input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        expected = "6"
    )]
    fn part_two(input @ Input { .. }: &Input) -> impl std::fmt::Display {
        input
            .walk(None)
            .1
            .into_iter()
            .filter(|&position| position != input.get_start())
            .filter(|&position| input.walk(Some(position)).0)
            .count()
    }
}
