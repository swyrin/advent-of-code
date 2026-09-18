use std::collections::HashSet;

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

type Position = (usize, usize);

struct Input {
    grid: Vec<Vec<u8>>,
    start: Position,
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

        let mut start = None;
        for (row, line) in grid.iter().enumerate() {
            for (column, &cell) in line.iter().enumerate() {
                if cell == b'^' {
                    start = Some((row, column));
                    break;
                }
            }

            if start.is_some() {
                break;
            }
        }

        let Some(start) = start else {
            return Err("guard map has no starting position".to_string());
        };

        Ok(Self {
            grid,
            start,
        })
    }
}

impl Input {
    fn cell(&self, (row, column): Position) -> Option<u8> {
        self.grid.get(row).and_then(|line| line.get(column)).copied()
    }

    fn walk(&self, extra_obstacle: Option<Position>) -> (bool, HashSet<Position>) {
        const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let mut position = self.start;
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

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input.walk(None).1.len())
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .walk(None)
        .1
        .into_iter()
        .filter(|&position| position != input.start)
        .filter(|&position| input.walk(Some(position)).0)
        .count())
}

#[cfg(test)]
mod test {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(input = { r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..." }, expected = { "41" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..." }, expected = { "6" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
