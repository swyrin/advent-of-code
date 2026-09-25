use counter::Counter;
use macros::{AocInput, aoc, part, sample};
use swiss_knife::common::math::bin2dec::bin_convert_to_dec_be;
use swiss_knife::common::math::linalg::transpose2;

#[derive(AocInput)]
struct Input {
    #[parse(lines(digit_bin+))]
    pub(crate) metrics: Vec<Vec<usize>>,
}
aoc!();

#[part]
#[sample(
    input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
    expected = "198"
)]
fn part_one(
    Input {
        metrics,
    }: &Input,
) -> impl std::fmt::Display {
    let transposed = transpose2(metrics.clone());

    let mut gamma_binaries: Vec<usize> = vec![];
    let mut epsil_binaries: Vec<usize> = vec![];

    for row in transposed {
        let counter = row.iter().collect::<Counter<_>>();

        let n0 = counter[&0usize];
        let n1 = counter[&1usize];

        if n1 >= n0 {
            gamma_binaries.push(1);
            epsil_binaries.push(0);
        } else {
            gamma_binaries.push(0);
            epsil_binaries.push(1);
        }
    }

    let gamma = bin_convert_to_dec_be(&gamma_binaries);
    let epsil = bin_convert_to_dec_be(&epsil_binaries);

    gamma * epsil
}

#[part]
#[sample(
    input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
    expected = "230"
)]
fn part_two(
    Input {
        metrics,
    }: &Input,
) -> impl std::fmt::Display {
    let transposed = transpose2(metrics.clone());

    let mut position = 0;
    let zeroth = transposed[0].clone();
    let counter = zeroth.iter().collect::<Counter<_>>();
    let n0 = counter[&0usize];
    let n1 = counter[&1usize];

    let mut o2: Vec<Vec<usize>> = vec![];
    let mut co2: Vec<Vec<usize>> = vec![];
    let dominant_value = if n1 >= n0 { 1 } else { 0 };

    for (index, val) in transposed[position].iter().enumerate() {
        let x = metrics.get(index).unwrap();

        if *val == dominant_value {
            o2.push(x.to_vec());
        } else {
            co2.push(x.to_vec());
        }
    }

    position = 1;

    while o2.len() > 1 {
        let temp_o2 = o2.clone();
        let transposed = transpose2(temp_o2.clone());

        let nth = transposed[position].clone();
        let counter = nth.iter().collect::<Counter<_>>();
        let n0 = counter[&0usize];
        let n1 = counter[&1usize];
        let dominant_value = if n1 >= n0 { 1 } else { 0 };

        let mut filtered_o2: Vec<Vec<usize>> = vec![];

        for (index, val) in nth.iter().enumerate() {
            let x = temp_o2.get(index).unwrap();

            if *val == dominant_value {
                filtered_o2.push(x.to_vec());
            }
        }

        position += 1;
        o2 = filtered_o2;
    }

    position = 1;

    while co2.len() > 1 {
        let temp_co2 = co2.clone();
        let transposed = transpose2(temp_co2.clone());

        let nth = transposed[position].clone();
        let counter = nth.iter().collect::<Counter<_>>();
        let n0 = counter[&0usize];
        let n1 = counter[&1usize];
        let least_dominant_value = if n0 <= n1 { 0 } else { 1 };

        let mut filtered_co2: Vec<Vec<usize>> = vec![];

        for (index, val) in nth.iter().enumerate() {
            let x = temp_co2.get(index).unwrap();

            if *val == least_dominant_value {
                filtered_co2.push(x.to_vec());
            }
        }

        position += 1;
        co2 = filtered_co2;
    }

    let gamma = bin_convert_to_dec_be(&o2[0]);
    let epsil = bin_convert_to_dec_be(&co2[0]);

    gamma * epsil
}
