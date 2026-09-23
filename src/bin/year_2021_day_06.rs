use std::collections::HashMap;

use macros::{AocInput, aoc};

#[derive(AocInput)]
struct Input {
    #[parse(line(repeat_sep(i64, ",")))]
    pub(crate) numbers: Vec<i64>,
}

// Why are all simulation days HashMap?
// Saved you a read: P1 is 80, P2 is 256
aoc! {
    #[sample(
        input = "3,4,3,1,2",
        expected = "5934"
    )]
    fn part_one(Input { numbers }: &Input) -> impl std::fmt::Display {
        // number -> count
        let mut fishes: HashMap<i64, i64> = HashMap::new();

        for fish in numbers {
            if let Some(val) = fishes.get_mut(fish) {
                *val += 1;
            } else {
                fishes.insert(*fish, 1);
            }
        }


        let mut day = 1;

        while day <= 80 {
            let mut next_fishes: HashMap<i64, i64> = HashMap::new();

            if let Some(count_zero) = fishes.get(&0) {
                next_fishes.insert(6, *count_zero);
                next_fishes.insert(8, *count_zero);
            }

            // too many headaches, so 0 is handled separately
            let mut fishes_without_zero = fishes.clone();

            if fishes_without_zero.contains_key(&0) {
                fishes_without_zero.remove(&0).unwrap();
            }

            for (key, count) in fishes_without_zero.iter() {
                let candidate = (*key) - 1;

                if let Some(count_existing) = next_fishes.get(&candidate) {
                    next_fishes.insert(candidate, *count_existing + *count);
                } else {
                    next_fishes.insert(candidate, *count);
                }
            }

            fishes = next_fishes;
            day += 1;
        };

        fishes.values().sum::<i64>()
    }

    #[sample(
        input = "3,4,3,1,2",
        expected = "26984457539"
    )]
    fn part_two(Input { numbers }: &Input) -> impl std::fmt::Display {
        // number -> count
        let mut fishes: HashMap<i64, i64> = HashMap::new();

        for fish in numbers {
            if let Some(val) = fishes.get_mut(fish) {
                *val += 1;
            } else {
                fishes.insert(*fish, 1);
            }
        }


        let mut day = 1;

        while day <= 256 {
            let mut next_fishes: HashMap<i64, i64> = HashMap::new();

            if let Some(count_zero) = fishes.get(&0) {
                next_fishes.insert(6, *count_zero);
                next_fishes.insert(8, *count_zero);
            }

            // too many headaches, so 0 is handled separately
            let mut fishes_without_zero = fishes.clone();

            if fishes_without_zero.contains_key(&0) {
                fishes_without_zero.remove(&0).unwrap();
            }

            for (key, count) in fishes_without_zero.iter() {
                let candidate = (*key) - 1;

                if let Some(count_existing) = next_fishes.get(&candidate) {
                    next_fishes.insert(candidate, *count_existing + *count);
                } else {
                    next_fishes.insert(candidate, *count);
                }
            }

            fishes = next_fishes;
            day += 1;
        };

        fishes.values().sum::<i64>()
    }
}
