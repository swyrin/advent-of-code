use std::collections::{HashMap, HashSet};

use itertools::Itertools;

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
    rows: isize,
    columns: isize,
    antennas: HashMap<char, Vec<Position>>,
}

impl std::str::FromStr for Input {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let rows = content
            .lines()
            .filter(|line| !line.is_empty())
            .map(str::chars)
            .map(|characters| characters.collect::<Vec<_>>())
            .collect::<Vec<_>>();

        if rows.is_empty() {
            return Err("invalid antenna map: empty".to_string());
        }

        if rows.iter().any(|row| row.len() != rows[0].len()) {
            return Err("invalid antenna map: ragged rows".to_string());
        }

        let mut antennas = HashMap::<char, Vec<Position>>::new();

        for (row, line) in rows.iter().enumerate() {
            for (column, &frequency) in line.iter().enumerate() {
                if frequency.is_ascii_alphanumeric() {
                    antennas
                        .entry(frequency)
                        .or_default()
                        .push((row as isize, column as isize));
                }
            }
        }

        Ok(Self {
            rows: rows.len() as isize,
            columns: rows[0].len() as isize,
            antennas,
        })
    }
}

impl Input {
    fn in_bounds(&self, (row, column): Position) -> bool {
        (0..self.rows).contains(&row) && (0..self.columns).contains(&column)
    }
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut antinodes = HashSet::new();

    for locations in input.antennas.values() {
        for [first, second] in locations.iter().copied().array_combinations() {
            let delta = (second.0 - first.0, second.1 - first.1);
            antinodes.insert((first.0 - delta.0, first.1 - delta.1));
            antinodes.insert((second.0 + delta.0, second.1 + delta.1));
        }
    }

    Ok(antinodes
        .into_iter()
        .filter(|&position| input.in_bounds(position))
        .count())
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut antinodes = HashSet::new();

    for locations in input.antennas.values() {
        for [first, second] in locations.iter().copied().array_combinations() {
            let delta = (second.0 - first.0, second.1 - first.1);

            let mut position = first;
            while input.in_bounds(position) {
                antinodes.insert(position);
                position = (position.0 - delta.0, position.1 - delta.1);
            }

            let mut position = second;
            while input.in_bounds(position) {
                antinodes.insert(position);
                position = (position.0 + delta.0, position.1 + delta.1);
            }
        }
    }

    Ok(antinodes.len())
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = { r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............" }, expected = { "14" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............" }, expected = { "34" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
