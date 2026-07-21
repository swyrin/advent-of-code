use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_libraries::image::{GrayImage, Luma};
use aoc_macros::aoc_submission;

const SAMPLE_DIMENSIONS: (i64, i64) = (11, 7);
const ACTUAL_DIMENSIONS: (i64, i64) = (101, 103);

#[derive(Clone, Copy)]
pub struct Robot {
    pub position: (i64, i64),
    pub velocity: (i64, i64),
}

pub struct Input {
    pub robots: Vec<Robot>,
    pub dimensions: (i64, i64),
}

impl std::str::FromStr for Input {
    type Err = aoc_libraries::aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let robots = parser!(lines("p=" i64 "," i64 " v=" i64 "," i64))
            .parse(content)?
            .into_iter()
            .map(|(x, y, velocity_x, velocity_y)| Robot {
                position: (x, y),
                velocity: (velocity_x, velocity_y),
            })
            .collect::<Vec<_>>();
        let dimensions = if robots.iter().all(|robot| {
            robot.position.0 < SAMPLE_DIMENSIONS.0 && robot.position.1 < SAMPLE_DIMENSIONS.1
        }) {
            SAMPLE_DIMENSIONS
        } else {
            ACTUAL_DIMENSIONS
        };

        Ok(Self { robots, dimensions })
    }
}

fn position_at(robot: Robot, seconds: i64, (width, height): (i64, i64)) -> (u32, u32) {
    (
        (robot.position.0 + robot.velocity.0 * seconds).rem_euclid(width) as u32,
        (robot.position.1 + robot.velocity.1 * seconds).rem_euclid(height) as u32,
    )
}

#[aoc_submission(
    sample_in = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
    sample_out = 12
)]
pub fn part_1(input: Input) -> impl std::fmt::Display {
    let mut quadrants = [0_u64; 4];
    let middle_x = (input.dimensions.0 / 2) as u32;
    let middle_y = (input.dimensions.1 / 2) as u32;

    for robot in input.robots {
        let (x, y) = position_at(robot, 100, input.dimensions);
        let quadrant = match (x.cmp(&middle_x), y.cmp(&middle_y)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(0),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(1),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(2),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Some(3),
            _ => None,
        };

        if let Some(quadrant) = quadrant {
            quadrants[quadrant] += 1;
        }
    }

    quadrants.into_iter().product::<u64>()
}

#[aoc_submission(sample_in = "", sample_out = "This is a joke.")]
pub fn part_2(input: Input) -> impl std::fmt::Display {
    let seconds = 7000;

    for second in 1..=seconds {
        let mut canvas = GrayImage::from_pixel(
            input.dimensions.0 as u32,
            input.dimensions.1 as u32,
            Luma([0]),
        );

        for robot in input.robots.clone() {
            let (x, y) = position_at(robot, second, input.dimensions);

            canvas.put_pixel(x, y, Luma([255]));
        }

        canvas
            .save(format!("images/output_{second}.png"))
            .expect("Unable to save image");
    }

    "This is a joke."
}
