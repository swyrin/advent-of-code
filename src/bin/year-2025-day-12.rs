use aoc_parse::{parser, prelude::*};

fn main() {
    use std::io::Read;

    let mut input_content = std::fs::File::open("input.txt").expect("No input.txt file");
    let mut buffer = String::new();

    input_content.read_to_string(&mut buffer).unwrap();
    let input: Input = buffer.as_str().parse().unwrap();

    let ans1 = part_one(&input);
    let ans2 = part_two(&input);

    if let Ok(ans1) = ans1 {
        println!("Result of part 1: {ans1}")
    } else {
        println!("Part 1 fails.")
    }

    if let Ok(ans2) = ans2 {
        println!("Result of part 2: {ans2}")
    } else {
        println!("Part 2 fails.")
    }
}

#[derive(Debug)]
struct Board {
    area: usize,
    piece_counts: Vec<usize>,
}

struct Input {
    brick_areas: Vec<usize>,
    boards: Vec<Board>,
}

impl std::str::FromStr for Input {
    type Err = aoc_parse::ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
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
        let (pieces, boards) = parser!(piece_parser board_parser).parse(content)?;
        let brick_areas = pieces
            .into_iter()
            .map(|(_, rows)| {
                rows.iter()
                    .map(|row| row.chars().filter(|&character| character == '#').count())
                    .sum()
            })
            .collect();

        Ok(Self {
            brick_areas,
            boards,
        })
    }
}

/// Eric put a troll problem.
#[forbid(unsafe_code)]
fn part_one(input: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .boards
        .iter()
        .filter(|board| {
            let required_area = board
                .piece_counts
                .iter()
                .zip(&input.brick_areas)
                .map(|(piece_count, brick_area)| piece_count * brick_area)
                .sum::<usize>();
            board.area > required_area
        })
        .count())
}

#[forbid(unsafe_code)]
fn part_two(_: &Input) -> anyhow::Result<impl std::fmt::Display> {
    Ok(42)
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = { r"0:
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
12x5: 1 0 1 0 3 2" }, expected = { "3" })]
    fn test_part_1(input: &str, expected: &str) {
        let input = input.parse().unwrap();
        let answer = part_one(&input);

        if let Ok(actual) = answer {
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    #[test]
    #[ignore = "Ho ho ho!"]
    fn test_part_2() {}
}
