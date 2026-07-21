use aoc_macros::aoc_submission;

pub struct Input {
    pub lines: Vec<String>,
}

impl std::str::FromStr for Input {
    type Err = std::convert::Infallible;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let lines = content.lines().map(|s| s.to_string()).collect();
        Ok(Self { lines })
    }
}

#[aoc_submission(
    sample_in = r"987654321111111
811111111111119
234234234234278
818181911112111",
    sample_out = 357
)]
fn part_1(input: Input) -> impl std::fmt::Display {
    let mut total: usize = 0;

    for line in input.lines {
        let numbers: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();

        // basically, a linear function mx + b, at its maxima when m and b reaches max.

        let index_max_1: usize = numbers
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .expect("Hmmmm.");

        let max_1 = numbers[index_max_1];

        let mut max_left = u32::MIN;
        let mut max_right = u32::MIN;

        for left_i in numbers.iter().take(index_max_1) {
            max_left = max_left.max(*left_i);
        }

        for right_i in numbers.iter().skip(index_max_1 + 1) {
            max_right = max_right.max(*right_i);
        }

        let mut result1 = max_left * 10 + max_1;
        let mut result2 = max_1 * 10 + max_right;

        if max_left == 0 {
            result1 = 0;
        }

        if max_right == 0 {
            result2 = 0;
        }

        let volt = u32::max(result1, result2);

        total += volt as usize;
    }

    total
}

#[aoc_submission(
    sample_in = r"987654321111111
811111111111119
234234234234278
818181911112111",
    sample_out = 3121910778619u128
)]
fn part_2(input: Input) -> impl std::fmt::Display {
    // can't believe we jumped from 2 to 12, smh.
    let mut total: i128 = 0;

    // basically, try to form the longest number chain possible,
    // like 9999..9, 999..8, ... and so on.
    // https://en.wikipedia.org/wiki/Tournament_sort
    // (sans the min-heap part).
    for line in input.lines {
        let numbers: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut arr: Vec<u32> = vec![];

        for (i, v) in numbers.iter().enumerate() {
            let num = *v;
            let obj = num;

            loop {
                // for some reason, leave this code above the `loop` doesn't work.
                if arr.is_empty() {
                    arr.push(obj);
                    break;
                    // continue;
                }

                let unused_number_count = line.len() - i;
                let slot_count = 12 - arr.len();
                let last = *arr.last().unwrap();

                // basically, clean up small numbers so that
                // the larger number (champion) will join the inner bracket.
                // to create a chain of 9's, then 8's, ...
                if last < num && unused_number_count > slot_count {
                    arr.pop();
                    continue;
                }

                // then the champion joins the bracket.
                if slot_count > 0 {
                    arr.push(num);
                    break;
                } else {
                    break;
                }
            }
        }

        if arr.is_empty() {
            panic!("Why are you panic?")
        }

        let number: i128 = arr.iter().fold(0, |umi, meow| umi * 10 + *meow as i128);

        total += number;
    }

    total
}
