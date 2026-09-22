use macros::{AocInput, aoc};

#[derive(AocInput)]
struct Input {
    #[parse(lines({
        "L" amount:i32 => ('L', amount),
        "R" amount:i32 => ('R', amount),
    }))]
    pub(crate) moves: Vec<(char, i32)>,
}

aoc! {
    #[sample(
        input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        expected = "3"
    )]
    fn part_one(Input { moves }: &Input) -> impl std::fmt::Display {
        let mut pos = 50;
        let mut count = 0;

        for &(direction, amount) in moves {
            match direction {
                'L' => {
                    pos = (pos - amount) % 100;
                },
                'R' => {
                    pos = (pos + amount) % 100;
                },
                _ => {
                    panic!("Not a valid direction.")
                },
            }

            if pos == 0 {
                count += 1;
            }
        }

        count
    }

    #[sample(
        input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        expected = "6"
    )]
    fn part_two(Input { moves }: &Input) -> impl std::fmt::Display {
        let mut pos: i64 = 50;
        let mut count: i64 = 0;

        for &(direction, amount) in moves {
            let amount = amount as i64;
            count += amount / 100;
            let amount = amount % 100;

            match direction {
                'L' => {
                    if pos != 0 && pos - amount <= 0 {
                        count += 1;
                    }

                    pos = (pos - amount).rem_euclid(100);
                },
                'R' => {
                    if pos != 0 && pos + amount >= 100 {
                        count += 1;
                    }

                    pos = (pos + amount).rem_euclid(100);
                },
                _ => {
                    panic!("Not a valid direction.")
                },
            }
        }

        count
    }
}
