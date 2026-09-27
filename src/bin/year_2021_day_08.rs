use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use macros::{AocInput, aoc, part, sample};

#[derive(AocInput)]
struct Input {
    #[parse(
        rule code: String = string(alpha+);
        rule codes: Vec<String> = repeat_sep(code, " ");

        lines(codes " | " codes)
    )]
    pub(crate) patterns: Vec<(Vec<String>, Vec<String>)>,
}

fn intersection<'a, I>(messages: I) -> HashSet<char>
where
    I: IntoIterator<Item = &'a String>,
{
    let mut iter = messages.into_iter();

    let Some(first) = iter.next() else {
        return HashSet::new();
    };

    let mut result: HashSet<char> = first.chars().collect();

    for message in iter {
        let chars: HashSet<char> = message.chars().collect();

        result.retain(|c| chars.contains(c));
    }

    result
}

aoc!();

#[part]
#[sample(
    input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    expected = "26"
)]
fn part_one(
    Input {
        patterns,
    }: &Input,
) -> impl std::fmt::Display {
    patterns
        .iter()
        .map(|(_, right)| right)
        .map(|string| {
            string
                .iter()
                .filter(|&x| match x.len() {
                    // 1
                    2 => true,
                    // 4
                    4 => true,
                    // 7
                    3 => true,
                    // 8
                    7 => true,
                    _ => false,
                })
                .count()
        })
        .sum::<usize>()
}

#[part]
#[sample(
    input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    expected = "61229"
)]
#[sample(
    input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
    expected = "5353"
)]
fn part_two(
    Input {
        patterns,
    }: &Input,
) -> impl std::fmt::Display {
    let mut total = 0usize;

    let digits = HashMap::from([
        ("abcefg", '0'),
        ("cf", '1'),
        ("acdeg", '2'),
        ("acdfg", '3'),
        ("bcdf", '4'),
        ("abdfg", '5'),
        ("abdefg", '6'),
        ("acf", '7'),
        ("abcdefg", '8'),
        ("abcdfg", '9'),
    ]);

    // 0:      1:      2:      3:      4:
    // aaaa    ....    aaaa    aaaa    ....
    // b    c  .    c  .    c  .    c  b    c
    // b    c  .    c  .    c  .    c  b    c
    // ....    ....    dddd    dddd    dddd
    // e    f  .    f  e    .  .    f  .    f
    // e    f  .    f  e    .  .    f  .    f
    // gggg    ....    gggg    gggg    ....
    //
    // 5:      6:      7:      8:      9:
    // aaaa    aaaa    aaaa    aaaa    aaaa
    // .b    .  b    .  .    c  b    c  b    c
    // b    .  b    .  .    c  b    c  b    c
    // dddd    dddd    ....    dddd    dddd
    // .    f  e    f  .    f  e    f  .    f
    // .    f  e    f  .    f  e    f  .    f
    // gggg    gggg    ....    gggg    gggg

    for (message, scrambles) in patterns {
        // &: intersect
        // \: exclude

        // I FUCKING LOVE SET THEORY

        // L=5 -> the burger
        let could235 = message.iter().filter(|message| message.len() == 5).collect_vec();
        let burger = intersection(could235);

        // the burger & 7 = top dash - a
        // (7 is the only one with L=3)
        let number7 = message.iter().filter(|message| message.len() == 3).collect_vec();

        let number7lines = intersection(number7);

        let top_dash: HashSet<_> = burger.intersection(&number7lines).copied().collect();

        // the burger & 4 = middle dash - d  (the only one with L=4)
        let number4 = message.iter().filter(|message| message.len() == 4).collect_vec();

        let number4lines = intersection(number4);

        let middle_dash: HashSet<_> = burger.intersection(&number4lines).copied().collect();

        // the burger \ top_dash \ middle_dash = bottom dash - g
        let top_middle_union: HashSet<_> = top_dash.union(&middle_dash).copied().collect();

        let bottom_dash: HashSet<_> = burger.difference(&top_middle_union).copied().collect();

        // 1 & (0, 6, 9 - L = 6) = top right - c
        let number1 =
            message.clone().into_iter().filter(|message| message.len() == 2).collect_vec();

        let number1lines = intersection(&number1);

        let could069 =
            message.clone().into_iter().filter(|message| message.len() == 6).collect_vec();

        let number069lines = intersection(&could069);

        let top_right: HashSet<_> = number1lines.difference(&number069lines).copied().collect();

        // 1 \ top_right                 = bottom right - f
        let bottom_right: HashSet<_> = number1lines.difference(&top_right).copied().collect();

        // 4 \ d \ c \ f        = top left    - b
        let known_number4: HashSet<_> =
            middle_dash.union(&top_right).chain(bottom_right.iter()).copied().collect();

        let top_left: HashSet<_> = number4lines.difference(&known_number4).copied().collect();

        // 8 \ a \ d \ g \ c \  = bottom left - e
        let number8 =
            message.clone().into_iter().filter(|message| message.len() == 7).collect_vec();

        let number8lines = intersection(&number8);

        let known_number8: HashSet<_> = top_dash
            .union(&top_left)
            .chain(top_right.iter())
            .chain(middle_dash.iter())
            .chain(bottom_right.iter())
            .chain(bottom_dash.iter())
            .copied()
            .collect();

        let bottom_left: HashSet<_> = number8lines.difference(&known_number8).copied().collect();

        let segment_map = HashMap::from([
            (*top_dash.iter().next().unwrap(), 'a'),
            (*top_left.iter().next().unwrap(), 'b'),
            (*top_right.iter().next().unwrap(), 'c'),
            (*middle_dash.iter().next().unwrap(), 'd'),
            (*bottom_left.iter().next().unwrap(), 'e'),
            (*bottom_right.iter().next().unwrap(), 'f'),
            (*bottom_dash.iter().next().unwrap(), 'g'),
        ]);

        eprintln!("{:?}", segment_map);

        let decoded: Vec<char> = scrambles
            .iter()
            .map(|scramble| {
                let normalized: String =
                    scramble.chars().map(|c| segment_map[&c]).sorted().collect();

                digits[normalized.as_str()]
            })
            .collect();

        let number: usize = decoded.iter().collect::<String>().parse().unwrap();

        total += number;
    }

    total
}
