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
    grid: Vec<Vec<char>>,
}

impl std::str::FromStr for Input {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let grid = content
            .lines()
            .map(str::chars)
            .map(|characters| characters.collect::<Vec<_>>())
            .collect::<Vec<_>>();

        if grid.is_empty() {
            return Err("invalid grid: empty".to_string());
        }

        if grid.iter().any(|row| row.len() != grid[0].len()) {
            return Err("invalid grid: ragged rows".to_string());
        }

        Ok(Self { grid })
    }
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

        DIRECTIONS
            .into_iter()
            .filter_map(move |(delta_row, delta_column)| {
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

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(removable_rolls(&input.grid).len())
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut grid = input.grid.clone();
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

    Ok(destroy_count)
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = { r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@." }, expected = { "13" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@." }, expected = { "43" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
