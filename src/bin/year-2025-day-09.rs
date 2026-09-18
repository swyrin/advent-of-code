use aoc_parse::parser;
use aoc_parse::prelude::*;
use geo::{Contains, LineString, Point, Polygon, Rect, point};
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

struct Input {
    points: Vec<(i128, i128)>,
}

impl std::str::FromStr for Input {
    type Err = aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let points = parser!(lines(i128 "," i128)).parse(content)?;

        Ok(Self {
            points,
        })
    }
}

#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .points
        .iter()
        .copied()
        .array_combinations()
        .map(|[(x1, y1), (x2, y2)]| {
            let width = x2.abs_diff(x1) as i128 + 1;
            let height = y2.abs_diff(y1) as i128 + 1;
            width * height
        })
        .max()
        .unwrap_or_default())
}

#[forbid(unsafe_code)]
fn part_two(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    let points: Vec<Point> =
        input.points.iter().map(|&(x, y)| point! { x: x as f64, y: y as f64 }).collect();

    let polygon = Polygon::new(LineString::from(points.clone()), vec![]);
    let mut max_area = 0_u128;

    for [a, b] in points.iter().array_combinations() {
        let rectangle = Rect::new(*a, *b);
        let (x1, y1) = a.x_y();
        let (x2, y2) = b.x_y();
        let area = (((x2 - x1).abs() + 1.0) * ((y2 - y1).abs() + 1.0)) as u128;

        if area > max_area && polygon.contains(&rectangle) {
            max_area = area;
        }
    }

    Ok(max_area)
}

#[cfg(test)]
mod test {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(input = { r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3" }, expected = { "50" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[parameterized(input = { r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3" }, expected = { "24" })]
    fn test_part_2(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_two(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
