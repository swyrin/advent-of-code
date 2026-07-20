use aoc_libraries::geo::{Contains, LineString, Point, Polygon, Rect, point};
use aoc_macros::aoc_submission;
use aoc_utils::traits::generalised_output::UmiAteTheOutput;
use aoc_utils::traits::parsable_input::ParsableInput;

pub struct Input {
    pub points: Vec<(i128, i128)>,
}

impl ParsableInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let points = content
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        Self { points }
    }
}

#[aoc_submission(
    input_type = crate::Input,
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
pub fn part_1(input: Input) -> UmiAteTheOutput {
    let points = input.points;
    let mut max_area = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let width = x2.abs_diff(x1) as i128 + 1;
            let height = y2.abs_diff(y1) as i128 + 1;
            max_area = max_area.max(width * height);
        }
    }

    UmiAteTheOutput::from_number(max_area)
}

#[aoc_submission(
    input_type = crate::Input,
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
pub fn part_2(input: Input) -> UmiAteTheOutput {
    let points: Vec<Point> = input
        .points
        .into_iter()
        .map(|(x, y)| point! { x: x as f64, y: y as f64 })
        .collect();

    let polygon = Polygon::new(LineString::from(points.clone()), vec![]);
    let mut max_area = 0_u128;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let rectangle = Rect::new(points[i], points[j]);
            let (x1, y1) = points[i].x_y();
            let (x2, y2) = points[j].x_y();
            let area = (((x2 - x1).abs() + 1.0) * ((y2 - y1).abs() + 1.0)) as u128;

            if area > max_area && polygon.contains(&rectangle) {
                max_area = area;
            }
        }
    }

    UmiAteTheOutput::from_number(max_area)
}
