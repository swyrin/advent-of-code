use aoc_libraries::aoc_parse::{parser, prelude::*};
use aoc_libraries::core::aoc_input::AocInput;
use aoc_libraries::core::aoc_output::AocOutput;
use aoc_macros::aoc_submission;

#[derive(Debug)]
pub struct Board {
    pub area: usize,
    pub piece_counts: Vec<usize>,
}

pub struct Input {
    pub brick_areas: Vec<usize>,
    pub boards: Vec<Board>,
}

impl AocInput for Input {
    fn from_raw_string(content: &str) -> Self {
        let piece_parser = parser!(sections(
            line(usize ":")
            lines(string(char_of(".#")+))
        ));
        let board_parser = parser!(lines(
            width:usize "x" height:usize ": "
            piece_counts:repeat_sep(usize, " ")
                => Board {
                    area: width * height,
                    piece_counts,
                }
        ));
        let (pieces, boards) = parser!(piece_parser board_parser).parse(content).unwrap();
        let brick_areas = pieces
            .into_iter()
            .map(|(_, rows)| {
                rows.iter()
                    .map(|row| row.chars().filter(|&character| character == '#').count())
                    .sum()
            })
            .collect();

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
pub fn part_1(input: Input) -> AocOutput {
    let result_count = input
        .boards
        .into_iter()
        .filter(|board| {
            let required_area = board
                .piece_counts
                .iter()
                .zip(&input.brick_areas)
                .map(|(piece_count, brick_area)| piece_count * brick_area)
                .sum::<usize>();
            board.area > required_area
        })
        .count();

    AocOutput::from_number(result_count)
}
