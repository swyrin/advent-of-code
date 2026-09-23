use macros::{AocInput, aoc};
use swiss_knife::common::math::linalg::transpose2;

#[derive(AocInput)]
struct Input {
    #[parse(section(line(repeat_sep(i32, ","))))]
    pub(crate) calls: Vec<i32>,
    #[parse(
        number = " " * n:i32 => n,
        board = lines(repeat_sep(number, " ")),
        sections(board)
    )]
    pub(crate) boards: Vec<Vec<Vec<i32>>>,
}

const MARKER: i32 = -1;

fn is_valid_bingo(board: Vec<Vec<i32>>) -> bool {
    let winning_pattern = vec![MARKER, MARKER, MARKER, MARKER, MARKER];

    for row in board.iter() {
        if *row == winning_pattern {
            return true;
        }
    }

    let transposed = transpose2(board.clone());

    for row in transposed.iter() {
        if *row == winning_pattern {
            return true;
        }
    }

    false
}

aoc! {
    #[sample(
        input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
        expected = "4512"
    )]
    fn part_one(Input { calls, boards }: &Input) -> impl std::fmt::Display {
        let mut boards = boards.clone();
        for &call in calls {
            for board in boards.iter_mut() {
                for row in board.iter_mut() {
                    for number in row.iter_mut() {
                        if *number == call {
                            *number = MARKER;
                        }
                    }
                }

                if is_valid_bingo(board.to_vec()) {
                    let winning_call = call;
                    let unmarked_sum: i32 =
                        board.iter().flat_map(|x| x.iter()).filter(|x| **x != MARKER).sum();

                    let result = winning_call * unmarked_sum;

                    return result;
                }
            }
        }

        panic!("How did we get here?")
    }

    #[sample(
        input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
        expected = "1924"
    )]
    fn part_two(Input { calls, boards }: &Input) -> impl std::fmt::Display {
        let mut boards = boards.clone();
        let mut winning_boards: Vec<usize> = vec![];
        let board_count = boards.len();

        for &call in calls {
            for (board_index, board) in boards.iter_mut().enumerate() {
                if winning_boards.contains(&board_index) {
                    continue;
                }

                for row in board.iter_mut() {
                    for number in row.iter_mut() {
                        if *number == call {
                            *number = MARKER;
                        }
                    }
                }

                if is_valid_bingo(board.to_vec()) {
                    if winning_boards.len() < board_count - 1 {
                        winning_boards.push(board_index);
                    } else {
                        let winning_call = call;
                        let unmarked_sum: i32 =
                            board.iter().flat_map(|x| x.iter()).filter(|x| **x != MARKER).sum();

                        let result = winning_call * unmarked_sum;

                        return result;
                    }
                }
            }
        }

        panic!("How did we get here?")
    }
}
