use macros::{AocInput, aoc};

#[derive(AocInput)]
struct Input {
    #[parse(rows:lines(string(any_char+)) => to_grid(rows))]
    grid: Vec<Vec<u8>>,
}

fn to_grid(rows: Vec<String>) -> Vec<Vec<u8>> {
    let grid = rows
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    if grid.is_empty() {
        panic!("invalid word-search grid: empty");
    }

    if grid.iter().any(|row| row.len() != grid[0].len()) {
        panic!("invalid word-search grid: ragged rows");
    }

    grid
}

impl Input {
    fn cells(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.grid.len())
            .flat_map(|row| (0..self.grid[row].len()).map(move |column| (row, column)))
    }
}

fn cell(grid: &[Vec<u8>], row: isize, column: isize) -> Option<u8> {
    if row < 0 || column < 0 {
        return None;
    }

    grid.get(row as usize).and_then(|line| line.get(column as usize)).copied()
}

aoc! {
    #[sample(
        input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        expected = "18"
    )]
    fn part_one(input @ Input { grid }: &Input) -> impl std::fmt::Display {
        const DIRECTIONS: [(isize, isize); 8] =
            [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        const WORD: &[u8] = b"XMAS";

        input
            .cells()
            .map(|(row, column)| {
                DIRECTIONS
                    .iter()
                    .filter(|&&(delta_row, delta_column)| {
                        WORD.iter().enumerate().all(|(offset, expected)| {
                            let offset = offset as isize;
                            cell(
                                grid,
                                row as isize + delta_row * offset,
                                column as isize + delta_column * offset,
                            ) == Some(*expected)
                        })
                    })
                    .count()
            })
            .sum::<usize>()
    }

    #[sample(
        input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        expected = "9"
    )]
    fn part_two(input @ Input { grid }: &Input) -> impl std::fmt::Display {
        input
            .cells()
            .filter(|&(row, column)| {
                if grid[row][column] != b'A' {
                    return false;
                }

                let diagonal = (
                    cell(grid, row as isize - 1, column as isize - 1),
                    cell(grid, row as isize + 1, column as isize + 1),
                );
                let anti_diagonal = (
                    cell(grid, row as isize - 1, column as isize + 1),
                    cell(grid, row as isize + 1, column as isize - 1),
                );
                let is_mas = |pair| matches!(pair, (Some(b'M'), Some(b'S')) | (Some(b'S'), Some(b'M')));

                is_mas(diagonal) && is_mas(anti_diagonal)
            })
            .count()
    }
}
