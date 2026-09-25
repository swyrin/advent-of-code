use counter::Counter;
use macros::{AocInput, aoc, part, sample};
use num::integer::gcd;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point(i32, i32);

#[derive(Clone, Debug)]
struct Segment {
    from: Point,
    to: Point,
}

impl Segment {
    fn is_straight(&self) -> bool {
        let Point(x1, y1) = self.from;
        let Point(x2, y2) = self.to;

        let is_horizontal = x1 == x2;
        let is_vertical = y1 == y2;

        is_horizontal || is_vertical
    }

    fn points(self) -> Vec<Point> {
        let Point(x1, y1) = self.from;
        let Point(x2, y2) = self.to;

        let dx = x2 - x1;
        let dy = y2 - y1;
        let steps = gcd(x2.abs_diff(x1), y2.abs_diff(y1)) as i32;

        let step_x = dx / steps;
        let step_y = dy / steps;

        (0..=steps).map(|order| Point(x1 + step_x * order, y1 + step_y * order)).collect()
    }
}

#[derive(AocInput)]
struct Input {
    #[parse(
        lines(
            x1:i32 "," y1:i32 " -> " x2:i32 "," y2:i32 => Segment {
                from: Point(x1, y1),
                to: Point(x2, y2)
            }
        )
    )]
    pub(crate) segments: Vec<Segment>,
}
aoc!();

#[part]
#[sample(
    input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
    expected = "5"
)]
fn part_one(
    Input {
        segments,
    }: &Input,
) -> impl std::fmt::Display {
    let mut points: Vec<Point> = vec![];

    for segment in segments {
        if segment.is_straight() {
            points.extend(segment.clone().points());
        }
    }

    let count = points.into_iter().collect::<Counter<_>>();

    count.values().filter(|&&count| count > 1).count()
}

#[part]
#[sample(
    input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
    expected = "12"
)]
fn part_two(
    Input {
        segments,
    }: &Input,
) -> impl std::fmt::Display {
    let mut points: Vec<Point> = vec![];

    for segment in segments {
        points.extend(segment.clone().points());
    }

    let count = points.into_iter().collect::<Counter<_>>();

    count.values().filter(|&&count| count > 1).count()
}
