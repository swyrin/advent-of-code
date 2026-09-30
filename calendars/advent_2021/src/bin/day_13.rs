use std::collections::HashSet;

use aoc_prelude::image::{GrayImage, Luma};
use macros::{AocInput, aoc, part, sample};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct Point {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

pub(crate) enum Fold {
    X(u32),
    Y(u32),
}

#[derive(AocInput)]
struct Input {
    #[parse(
        section(lines(x:u32 "," y:u32 => Point { x, y }))
    )]
    pub(crate) dots: Vec<Point>,
    #[parse(
        section(lines({
            "fold along x=" x_val:u32 => Fold::X(x_val),
            "fold along y=" y_val:u32 => Fold::Y(y_val)
        }))
    )]
    pub(crate) folds: Vec<Fold>,
}

aoc!();

fn size_wh(points: &[Point]) -> (u32, u32) {
    let (mut w, mut h) = (u32::MIN, u32::MIN);

    for &point in points {
        w = w.max(point.x);
        h = h.max(point.y);
    }

    (w, h)
}

#[part]
#[sample(
    input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7"
    expected = "17"
)]
fn part_one(
    Input {
        dots,
        folds,
    }: &Input,
) -> impl std::fmt::Display {
    let mut dots = dots.clone();

    // part 1 just concern only the first fold.
    for fold in folds.iter().take(1) {
        // |------------cut_dim---------------------------original_dim|
        // |   intact      |             process_set                  |
        let (intact, mut process_set) = match fold {
            Fold::X(v) => {
                let unfolded: Vec<_> = dots.iter().filter(|&point| &point.x < v).copied().collect();
                let folded: Vec<_> = dots.iter().filter(|&point| &point.x > v).copied().collect();

                (unfolded, folded)
            },
            Fold::Y(v) => {
                let unfolded: Vec<_> = dots.iter().filter(|&point| &point.y < v).copied().collect();
                let folded: Vec<_> = dots.iter().filter(|&point| &point.y > v).copied().collect();

                (unfolded, folded)
            },
        };

        let (intact_width, intact_height) = size_wh(&intact);

        for point in process_set.iter_mut() {
            match fold {
                // tl;dw: fold "in half" the dots cords should be
                // within 2 * fold_point
                //
                // subtraction has a very cool property is that
                // if a and b are "symmetric" through the middle
                // one on the number line of [0; X]
                //
                // 1. X - a = b
                // 2. X - b = a
                //
                // may introduce negatives, then get wrapped due to
                // the numbers being unsigned so there exists `retain()`
                Fold::X(cut_x) => {
                    point.x = 2 * cut_x - point.x;
                },
                Fold::Y(cut_y) => {
                    point.y = 2 * cut_y - point.y;
                },
            }
        }

        process_set.retain(|point| point.x <= intact_width && point.y <= intact_height);

        let intact_set: HashSet<Point> = HashSet::from_iter(intact);
        let process_set: HashSet<Point> = HashSet::from_iter(process_set);

        dots = intact_set.union(&process_set).copied().collect();
    }

    dots.len()
}

#[part]
#[sample(
    input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5",
    expected = "16"
)]
fn full_sample_test(
    Input {
        dots,
        folds,
    }: &Input,
) -> impl std::fmt::Display {
    let mut dots = dots.clone();

    for fold in folds.iter() {
        // |------------cut_dim---------------------------original_dim|
        // |   intact      |             process_set                  |
        let (intact, mut process_set) = match fold {
            Fold::X(v) => {
                let unfolded: Vec<_> = dots.iter().filter(|&point| &point.x < v).copied().collect();
                let folded: Vec<_> = dots.iter().filter(|&point| &point.x > v).copied().collect();

                (unfolded, folded)
            },
            Fold::Y(v) => {
                let unfolded: Vec<_> = dots.iter().filter(|&point| &point.y < v).copied().collect();
                let folded: Vec<_> = dots.iter().filter(|&point| &point.y > v).copied().collect();

                (unfolded, folded)
            },
        };

        let (intact_width, intact_height) = size_wh(&intact);

        for point in process_set.iter_mut() {
            match fold {
                // tl;dw: fold "in half" the dots cords should be
                // within 2 * fold_point
                //
                // subtraction has a very cool property is that
                // if a and b are "symmetric" through the middle
                // one on the number line of [0; X]
                //
                // 1. X - a = b
                // 2. X - b = a
                //
                // may introduce negatives, then get wrapped due to
                // the numbers being unsigned so there exists `retain()`
                Fold::X(cut_x) => {
                    point.x = 2 * cut_x - point.x;
                },
                Fold::Y(cut_y) => {
                    point.y = 2 * cut_y - point.y;
                },
            }
        }

        process_set.retain(|point| point.x <= intact_width && point.y <= intact_height);

        let intact_set: HashSet<Point> = HashSet::from_iter(intact);
        let process_set: HashSet<Point> = HashSet::from_iter(process_set);

        dots = intact_set.union(&process_set).copied().collect();
    }

    dots.len()
}

#[part]
fn part_two(
    Input {
        dots,
        folds,
    }: &Input,
) -> impl std::fmt::Display {
    let mut dots = dots.clone();

    for fold in folds.iter() {
        // |------------cut_dim---------------------------original_dim|
        // |   intact      |             process_set                  |
        let (intact, mut process_set) = match fold {
            Fold::X(v) => {
                let unfolded: Vec<_> = dots.iter().filter(|&point| &point.x < v).copied().collect();
                let folded: Vec<_> = dots.iter().filter(|&point| &point.x > v).copied().collect();

                (unfolded, folded)
            },
            Fold::Y(v) => {
                let unfolded: Vec<_> = dots.iter().filter(|&point| &point.y < v).copied().collect();
                let folded: Vec<_> = dots.iter().filter(|&point| &point.y > v).copied().collect();

                (unfolded, folded)
            },
        };

        let (intact_width, intact_height) = size_wh(&intact);

        for point in process_set.iter_mut() {
            match fold {
                // tl;dw: fold "in half" the dots cords should be
                // within 2 * fold_point
                //
                // subtraction has a very cool property is that
                // if a and b are "symmetric" through the middle
                // one on the number line of [0; X]
                //
                // 1. X - a = b
                // 2. X - b = a
                //
                // may introduce negatives, then get wrapped due to
                // the numbers being unsigned so there exists `retain()`
                Fold::X(cut_x) => {
                    point.x = 2 * cut_x - point.x;
                },
                Fold::Y(cut_y) => {
                    point.y = 2 * cut_y - point.y;
                },
            }
        }

        process_set.retain(|point| point.x <= intact_width && point.y <= intact_height);

        let intact_set: HashSet<Point> = HashSet::from_iter(intact);
        let process_set: HashSet<Point> = HashSet::from_iter(process_set);

        dots = intact_set.union(&process_set).copied().collect();
    }

    let mut canvas = GrayImage::from_pixel(64, 64, Luma([0]));

    for dot in dots {
        canvas.put_pixel(dot.x, dot.y, Luma([255]));
    }

    canvas.save("images/year_2021/day_13.png").expect("Unable to save image");

    "see images/year_2021/day_13.png"
}
