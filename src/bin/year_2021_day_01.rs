use macros::{AocInput, aoc, part, sample};

#[derive(AocInput)]
struct Input {
    #[parse(lines(u32))]
    pub(crate) numbers: Vec<u32>,
}

aoc!();

#[part]
#[sample(
    input = "199
200
208
210
200
207
240
269
260
263",
    expected = "7"
)]
fn part_one(
    Input {
        numbers,
    }: &Input,
) -> impl std::fmt::Display {
    let mut increases = 0;

    let mut previous = *numbers.first().unwrap();

    for number in numbers.iter().skip(1) {
        if (*number) > previous {
            increases += 1;
        }

        previous = *number;
    }

    increases
}

#[part]
#[sample(
    input = "199
200
208
210
200
207
240
269
260
263",
    expected = "5"
)]
fn part_two(
    Input {
        numbers,
    }: &Input,
) -> impl std::fmt::Display {
    let mut increases = 0;

    let mut number1 = *numbers.first().unwrap();
    let mut number2 = *numbers.get(1).unwrap();
    let mut number3 = *numbers.get(2).unwrap();
    let mut sum = number1 + number2 + number3;

    for number in numbers.iter().skip(3) {
        number1 = number2;
        number2 = number3;
        number3 = *number;

        if number1 + number2 + number3 > sum {
            increases += 1;
        }

        sum = number1 + number2 + number3;
    }

    increases
}
