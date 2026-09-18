use std::collections::{HashMap, HashSet};

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
    content: String,
}

impl std::str::FromStr for Input {
    type Err = std::convert::Infallible;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            content: content.to_string(),
        })
    }
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let mut required_columns = HashSet::new();
    let lines = input.content.lines();

    for line in lines.clone().take(1) {
        for (column, character) in line.chars().enumerate() {
            if character == 'S' {
                required_columns.insert(column);
                break;
            }
        }
    }

    let mut total = 0_u128;

    for line in lines.skip(2).step_by(2) {
        let mut processed_columns = HashSet::new();
        let mut next_required_columns = HashSet::new();

        for (column, character) in line.chars().enumerate() {
            if character == '^' && required_columns.contains(&column) {
                processed_columns.insert(column);
                total += 1;
                next_required_columns.insert(column - 1);
                next_required_columns.insert(column + 1);
            }
        }

        for column in processed_columns {
            required_columns.remove(&column);
        }

        required_columns.extend(next_required_columns);
    }

    Ok(total)
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    // how many rays reach x
    let mut rays: HashMap<usize, u128> = HashMap::new();
    let lines = input.content.lines();

    for line in lines.clone().take(1) {
        for (column, character) in line.chars().enumerate() {
            if character == 'S' {
                rays.insert(column, 1);
                break;
            }
        }
    }

    for line in lines.skip(2).step_by(2) {
        let mut next_rays = HashMap::new();

        for (column, character) in line.chars().enumerate() {
            if let Some(ray_count) = rays.get(&column) {
                if character == '^' {
                    *next_rays.entry(column + 1).or_insert(0) += ray_count;
                    *next_rays.entry(column - 1).or_insert(0) += ray_count;
                } else {
                    *next_rays.entry(column).or_insert(0) += ray_count;
                }
            }
        }

        rays = next_rays;
    }

    Ok(rays.values().copied().sum::<u128>())
}

#[cfg(test)]
mod test {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(input = { r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............." }, expected = { "21" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............." }, expected = { "40" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
