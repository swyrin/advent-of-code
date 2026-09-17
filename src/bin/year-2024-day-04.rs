fn main() {
    use std::io::Read;

    let mut input_content = std::fs::File::open("input.txt").expect("No input.txt file");
    let mut buffer = String::new();

    input_content.read_to_string(&mut buffer).unwrap();
    let input: Input = buffer.as_str().parse().unwrap();

    let ans1 = part_one(&input);
    let ans2 = part_two(&input);

    if let Ok(ans1) = ans1 {
        println!("Result of part 1: {ans1}")
    } else {
        println!("Part 1 fails.")
    }

    if let Ok(ans2) = ans2 {
        println!("Result of part 2: {ans2}")
    } else {
        println!("Part 2 fails.")
    }
}

struct Input {
    grid: Vec<Vec<u8>>,
}

impl std::str::FromStr for Input {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let grid = content
            .lines()
            .filter(|line| !line.is_empty())
            .map(str::bytes)
            .map(|bytes| bytes.collect::<Vec<_>>())
            .collect::<Vec<_>>();

        if grid.is_empty() {
            return Err("invalid word-search grid: empty".to_string());
        }

        if grid.iter().any(|row| row.len() != grid[0].len()) {
            return Err("invalid word-search grid: ragged rows".to_string());
        }

        Ok(Self { grid })
    }
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

    grid.get(row as usize)
        .and_then(|line| line.get(column as usize))
        .copied()
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
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

    Ok(input
        .cells()
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
        .sum::<usize>())
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .cells()
        .filter(|&(row, column)| {
            if input.grid[row][column] != b'A' {
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
        .count())
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = { r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX" }, expected = { "18" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX" }, expected = { "9" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
