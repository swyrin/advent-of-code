use macros::{AocInput, aoc};

#[derive(AocInput)]
struct Input {
    #[parse(rows:lines(string(any_char+)) => to_grid(rows))]
    grid: Vec<Vec<char>>,
}

fn to_grid(rows: Vec<String>) -> Vec<Vec<char>> {
    let grid = rows.into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    if grid.is_empty() {
        panic!("invalid grid: empty");
    }

    if grid.iter().any(|row| row.len() != grid[0].len()) {
        panic!("invalid grid: ragged rows");
    }

    grid
}

impl Input {
    fn cells(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.grid.len())
            .flat_map(|row| (0..self.grid[row].len()).map(move |column| (row, column)))
    }

    fn neighbours(
        &self,
        (row, column): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        const DIRECTIONS: [(isize, isize); 8] =
            [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        DIRECTIONS.into_iter().filter_map(move |(delta_row, delta_column)| {
            let row = row as isize + delta_row;
            let column = column as isize + delta_column;

            if row < 0 || column < 0 {
                return None;
            }

            let (row, column) = (row as usize, column as usize);
            self.grid.get(row)?.get(column)?;
            Some((row, column))
        })
    }
}

fn removable_rolls(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let input = Input {
        grid: grid.to_vec(),
    };
    input
        .cells()
        .filter(|&(row, column)| {
            grid[row][column] == '@'
                && input
                    .neighbours((row, column))
                    .filter(|&(neighbour_row, neighbour_column)| {
                        grid[neighbour_row][neighbour_column] == '@'
                    })
                    .count()
                    < 4
        })
        .collect()
}

aoc! {
    #[sample(
        input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        expected = "13"
    )]
    fn part_one(Input { grid }: &Input) -> impl std::fmt::Display {
        removable_rolls(grid).len()
    }

    #[sample(
        input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        expected = "43"
    )]
    fn part_two(Input { grid }: &Input) -> impl std::fmt::Display {
        let mut grid = grid.clone();
        let mut destroy_count = 0;

        loop {
            let rolls = removable_rolls(&grid);

            if rolls.is_empty() {
                break;
            }

            destroy_count += rolls.len();
            for (row, column) in rolls {
                grid[row][column] = '.';
            }
        }

        destroy_count
    }
}
