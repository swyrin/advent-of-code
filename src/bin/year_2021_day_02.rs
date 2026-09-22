use macros::{AocInput, aoc};

use crate::Instructions::{Down, Forward, Up};

#[derive(Debug, Clone)]
enum Instructions {
    Forward(u32),
    Up(u32),
    Down(u32),
}

#[derive(AocInput)]
struct Input {
    #[parse(
        lines({
            "forward " step:u32 => Forward(step),
            "up " step:u32 => Up(step),
            "down " step:u32 => Down(step)
        })
    )]
    pub(crate) instructions: Vec<Instructions>,
}

aoc! {
    #[sample(
        input = "forward 5
down 5
forward 8
up 3
down 8
forward 2",
        expected = "150"
    )]
    fn part_one(Input { instructions }: &Input) -> impl std::fmt::Display {
        let mut horizontal_changes: Vec<i32> = vec![];
        let mut depth_changes: Vec<i32> = vec![];

        for inst in instructions {
            match *inst {
                Up(x) => depth_changes.push(-(x as i32)),
                Down(x) => depth_changes.push(x as i32),
                Forward(x) => horizontal_changes.push(x as i32),
            }
        }

        let horizontal: i32 = horizontal_changes.iter().sum();
        let depth: i32 = depth_changes.iter().sum();

        (horizontal * depth).to_string()
    }

    #[sample(
        input = "forward 5
down 5
forward 8
up 3
down 8
forward 2",
        expected = "900"
    )]
    fn part_two(Input { instructions }: &Input) -> impl std::fmt::Display {
        let mut aim = 0;
        let mut horizontal_changes: Vec<i32> = vec![];
        let mut depth_changes: Vec<i32> = vec![];

        for inst in instructions {
            match *inst {
                Up(x) => aim -= x as i32,
                Down(x) => aim += x as i32,
                Forward(x) => {
                    horizontal_changes.push(x as i32);
                    depth_changes.push((x as i32) * aim);
                },
            }
        }

        let horizontal: i32 = horizontal_changes.iter().sum();
        let depth: i32 = depth_changes.iter().sum();

        (horizontal * depth).to_string()
    }
}
