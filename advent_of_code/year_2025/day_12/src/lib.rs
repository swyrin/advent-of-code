use aoc_macros::aoc_submission;
use aoc_utils::traits::generalised_output::UmiAteTheOutput;
use aoc_utils::traits::parsable_input::ParsableInput;

#[derive(Debug)]
pub struct Board {
    pub area: u32,
    pub piece_counts: Vec<u32>,
}

pub struct Input {
    pub brick_areas: Vec<u32>,
    pub boards: Vec<Board>,
}

impl ParsableInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let mut brick_areas = vec![];
        let mut boards = vec![];
        let mut current_brick_area = 0;

        for line in content.lines() {
            if let Some((dimensions, counts)) = line.split_once(": ") {
                if current_brick_area > 0 {
                    brick_areas.push(current_brick_area);
                    current_brick_area = 0;
                }

                let (width, height) = dimensions.split_once('x').unwrap();
                let area = width.parse::<u32>().unwrap() * height.parse::<u32>().unwrap();
                let piece_counts = counts
                    .split_whitespace()
                    .map(|count| count.parse().unwrap())
                    .collect();
                boards.push(Board { area, piece_counts });
            } else if line.ends_with(':') {
                if current_brick_area > 0 {
                    brick_areas.push(current_brick_area);
                    current_brick_area = 0;
                }
            } else {
                current_brick_area +=
                    line.chars().filter(|&character| character == '#').count() as u32;
            }
        }

        if current_brick_area > 0 {
            brick_areas.push(current_brick_area);
        }

        Self {
            brick_areas,
            boards,
        }
    }
}

/// Eric put a troll problem.
#[aoc_submission(
    input_type = crate::Input,
    sample_in = r"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2",
    sample_out = 3
)]
pub fn part_1(input: Input) -> UmiAteTheOutput {
    let result_count = input
        .boards
        .into_iter()
        .filter(|board| {
            let required_area = board
                .piece_counts
                .iter()
                .zip(&input.brick_areas)
                .map(|(piece_count, brick_area)| piece_count * brick_area)
                .sum::<u32>();
            board.area > required_area
        })
        .count();

    UmiAteTheOutput::from_number(result_count)
}
