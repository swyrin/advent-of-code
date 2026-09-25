use geo::{Contains, LineString, Point, Polygon, Rect, point};
use itertools::Itertools;
use macros::{AocInput, aoc, part, sample};

#[derive(AocInput)]
struct Input {
    #[parse(lines(i128 "," i128))]
    pub(crate) points: Vec<(i128, i128)>,
}
aoc!();

#[part]
#[sample(
    input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
    expected = "50"
)]
fn part_one(
    Input {
        points,
    }: &Input,
) -> impl std::fmt::Display {
    points
        .iter()
        .copied()
        .array_combinations()
        .map(|[(x1, y1), (x2, y2)]| {
            let width = x2.abs_diff(x1) as i128 + 1;
            let height = y2.abs_diff(y1) as i128 + 1;
            width * height
        })
        .max()
        .unwrap_or_default()
}

#[part]
#[sample(
    input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
    expected = "24"
)]
fn part_two(
    Input {
        points,
    }: &Input,
) -> impl std::fmt::Display {
    let points: Vec<Point> =
        points.iter().map(|&(x, y)| point! { x: x as f64, y: y as f64 }).collect();

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
