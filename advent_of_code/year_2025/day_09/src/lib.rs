use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_libraries::core::aoc_input::AocInput;
use aoc_libraries::geo::{Contains, LineString, Point, Polygon, Rect, point};
use aoc_libraries::itertools::Itertools;
use aoc_macros::aoc_submission;

pub struct Input {
    pub points: Vec<(i128, i128)>,
}

impl AocInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let points = parser!(lines(i128 "," i128)).parse(content).unwrap();

        Self { points }
    }
}

#[aoc_submission(
    sample_in = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
    sample_out = 50
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    input
        .points
        .into_iter()
        .array_combinations()
        .map(|[(x1, y1), (x2, y2)]| {
            let width = x2.abs_diff(x1) as i128 + 1;
            let height = y2.abs_diff(y1) as i128 + 1;
            width * height
        })
        .max()
        .unwrap_or_default()
}

#[aoc_submission(
    sample_in = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
    sample_out = 24
)]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    let points: Vec<Point> = input
        .points
        .into_iter()
        .map(|(x, y)| point! { x: x as f64, y: y as f64 })
        .collect();

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

    max_area
}
