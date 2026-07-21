use aoc_libraries::pathfinding::matrix::Matrix;
use aoc_macros::aoc_submission;

pub struct Input {
    pub grid: Matrix<u8>,
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
        .map_err(|error| format!("invalid word-search grid: {error:?}"))?;

        Ok(Self { grid })
    }
}

fn cell(grid: &Matrix<u8>, row: isize, column: isize) -> Option<u8> {
    let row = usize::try_from(row).ok()?;
    let column = usize::try_from(column).ok()?;
    grid.get((row, column)).copied()
}

#[aoc_submission(
    sample_in = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    sample_out = 18
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    const WORD: &[u8] = b"XMAS";

    input
        .grid
        .keys()
        .map(|(row, column)| {
            DIRECTIONS
                .iter()
                .filter(|&&(delta_row, delta_column)| {
                    WORD.iter().enumerate().all(|(offset, expected)| {
                        let offset = offset as isize;
                        cell(
                            &input.grid,
                            row as isize + delta_row * offset,
                            column as isize + delta_column * offset,
                        ) == Some(*expected)
                    })
                })
                .count()
        })
        .sum::<usize>()
}

#[aoc_submission(
    sample_in = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    sample_out = 9
)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    input
        .grid
        .keys()
        .filter(|&(row, column)| {
            if input.grid[(row, column)] != b'A' {
                return false;
            }

            let diagonal = (
                cell(&input.grid, row as isize - 1, column as isize - 1),
                cell(&input.grid, row as isize + 1, column as isize + 1),
            );
            let anti_diagonal = (
                cell(&input.grid, row as isize - 1, column as isize + 1),
                cell(&input.grid, row as isize + 1, column as isize - 1),
            );
            let is_mas = |pair| matches!(pair, (Some(b'M'), Some(b'S')) | (Some(b'S'), Some(b'M')));

            is_mas(diagonal) && is_mas(anti_diagonal)
        })
        .count()
}
