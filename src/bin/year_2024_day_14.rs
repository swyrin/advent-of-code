use image::{GrayImage, Luma};
use macros::{AocInput, aoc};

const SAMPLE_DIMENSIONS: (i64, i64) = (11, 7);
const ACTUAL_DIMENSIONS: (i64, i64) = (101, 103);

#[derive(Clone, Copy, Debug)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

#[derive(AocInput)]
struct Input {
    #[parse(lines(
        "p=" x:i64 "," y:i64 " v=" vx:i64 "," vy:i64
            => Robot { position: (x, y), velocity: (vx, vy) }
    ))]
    pub(crate) robots: Vec<Robot>,
}

impl Input {
    fn dimensions(&self) -> (i64, i64) {
        if self.robots.iter().all(|robot| {
            robot.position.0 < SAMPLE_DIMENSIONS.0 && robot.position.1 < SAMPLE_DIMENSIONS.1
        }) {
            SAMPLE_DIMENSIONS
        } else {
            ACTUAL_DIMENSIONS
        }
    }
}

fn position_at(robot: Robot, seconds: i64, (width, height): (i64, i64)) -> (u32, u32) {
    (
        (robot.position.0 + robot.velocity.0 * seconds).rem_euclid(width) as u32,
        (robot.position.1 + robot.velocity.1 * seconds).rem_euclid(height) as u32,
    )
}

aoc! {
    #[sample(
        input = "p=0,4 v=3,-3
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
        expected = "12"
    )]
    fn part_one(input @ Input { robots }: &Input) -> impl std::fmt::Display {
        let dimensions = input.dimensions();
        let mut quadrants = [0_u64; 4];
        let middle_x = (dimensions.0 / 2) as u32;
        let middle_y = (dimensions.1 / 2) as u32;

        for &robot in robots {
            let (x, y) = position_at(robot, 100, dimensions);
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

    fn part_two(input @ Input { robots }: &Input) -> impl std::fmt::Display {
        let dimensions = input.dimensions();
        let seconds = 7000;

        for second in 1..=seconds {
            let mut canvas =
                GrayImage::from_pixel(dimensions.0 as u32, dimensions.1 as u32, Luma([0]));

            for &robot in robots {
                let (x, y) = position_at(robot, second, dimensions);

                canvas.put_pixel(x, y, Luma([255]));
            }

            canvas.save(format!("images/output_{second}.png")).expect("Unable to save image");
        }

        "This is a joke."
    }
}
