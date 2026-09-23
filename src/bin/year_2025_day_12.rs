use macros::{AocInput, aoc};

#[derive(Debug, Clone)]
struct Board {
    area: usize,
    piece_counts: Vec<usize>,
}

#[derive(AocInput)]
struct Input {
    #[parse(sections(
        line(usize ":")
        lines(string(char_of(".#")+))
    ))]
    pieces: Vec<(usize, Vec<String>)>,
    #[parse(section(lines(
        width:usize "x" height:usize ": "
        piece_counts:repeat_sep(usize, " ")
            => Board {
                area: width * height,
                piece_counts,
            }
    )))]
    boards: Vec<Board>,
}

impl Input {
    fn brick_areas(&self) -> Vec<usize> {
        self.pieces
            .iter()
            .map(|(_, rows)| {
                rows.iter()
                    .map(|row| row.chars().filter(|&character| character == '#').count())
                    .sum()
            })
            .collect()
    }
}

// Eric put a troll problem.
aoc! {
    #[sample(
        input = "0:
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
        expected = "3"
    )]
    fn part_one(input @ Input { boards, .. }: &Input) -> impl std::fmt::Display {
        let brick_areas = input.brick_areas();

        boards
            .iter()
            .filter(|board| {
                let required_area = board
                    .piece_counts
                    .iter()
                    .zip(&brick_areas)
                    .map(|(piece_count, brick_area)| piece_count * brick_area)
                    .sum::<usize>();
                board.area > required_area
            })
            .count()
    }
}
