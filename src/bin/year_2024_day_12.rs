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

type Position = (isize, isize);

struct Input {
    garden: Vec<Vec<u8>>,
}

impl std::str::FromStr for Input {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let garden = content
            .lines()
            .filter(|line| !line.is_empty())
            .map(str::bytes)
            .map(|bytes| bytes.collect::<Vec<_>>())
            .collect::<Vec<_>>();

        if garden.is_empty() {
            return Err("invalid garden map: empty".to_string());
        }

        if garden.iter().any(|row| row.len() != garden[0].len()) {
            return Err("invalid garden map: ragged rows".to_string());
        }

        Ok(Self {
            garden,
        })
    }
}

impl Input {
    fn plant(&self, (row, column): Position) -> Option<u8> {
        if row < 0 || column < 0 {
            return None;
        }

        self.garden.get(row as usize).and_then(|line| line.get(column as usize)).copied()
    }

    fn regions(&self) -> Vec<HashSet<Position>> {
        const DIRECTIONS: [Position; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut visited = HashSet::new();
        let mut regions = Vec::new();

        for row in 0..self.garden.len() {
            for column in 0..self.garden[row].len() {
                let start = (row as isize, column as isize);
                if !visited.insert(start) {
                    continue;
                }

                let plant = self.garden[row][column];
                let mut region = HashSet::from([start]);
                let mut stack = vec![start];

                while let Some(position) = stack.pop() {
                    for direction in DIRECTIONS {
                        let neighbour = (position.0 + direction.0, position.1 + direction.1);
                        if self.plant(neighbour) == Some(plant) && visited.insert(neighbour) {
                            region.insert(neighbour);
                            stack.push(neighbour);
                        }
                    }
                }

                regions.push(region);
            }
        }

        regions
    }
}

fn perimeter(region: &HashSet<Position>) -> usize {
    const DIRECTIONS: [Position; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    region
        .iter()
        .map(|&(row, column)| {
            DIRECTIONS
                .iter()
                .filter(|&&(delta_row, delta_column)| {
                    !region.contains(&(row + delta_row, column + delta_column))
                })
                .count()
        })
        .sum()
}

fn side_count(region: &HashSet<Position>) -> usize {
    const CORNERS: [(Position, Position); 4] =
        [((-1, 0), (0, -1)), ((-1, 0), (0, 1)), ((1, 0), (0, -1)), ((1, 0), (0, 1))];

    region
        .iter()
        .map(|&(row, column)| {
            CORNERS
                .iter()
                .filter(|&&(first, second)| {
                    let first = (row + first.0, column + first.1);
                    let second = (row + second.0, column + second.1);
                    let diagonal = (first.0 + second.0 - row, first.1 + second.1 - column);
                    let has_first = region.contains(&first);
                    let has_second = region.contains(&second);

                    (!has_first && !has_second)
                        || (has_first && has_second && !region.contains(&diagonal))
                })
                .count()
        })
        .sum()
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input.regions().into_iter().map(|region| region.len() * perimeter(&region)).sum::<usize>())
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input.regions().into_iter().map(|region| region.len() * side_count(&region)).sum::<usize>())
}

#[cfg(test)]
mod test {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(input = { r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE" }, expected = { "1930" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE" }, expected = { "1206" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
