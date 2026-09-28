use macros::{AocInput, aoc, part, sample};

#[derive(AocInput)]
struct Input {
    #[parse(lines(digit+))]
    pub(crate) grid: Vec<Vec<usize>>,
}

aoc!();

#[part]
#[sample(
    input = "2199943210
3987894921
9856789892
8767896789
9899965678",
    expected = "15"
)]
fn part_one(
    Input {
        grid,
    }: &Input,
) -> impl std::fmt::Display {
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut total = 0;

    for row in 0..nrows {
        'outer: for col in 0..ncols {
            let height = grid[row][col];

            let deltas: Vec<(i64, i64)> = vec![
                (row as i64 - 1, col as i64),
                (row as i64 + 1, col as i64),
                (row as i64, col as i64 - 1),
                (row as i64, col as i64 + 1),
            ];

            for (new_row, new_col) in deltas {
                if new_row < 0 || new_col < 0 {
                    continue;
                }

                if let Some(r) = grid.get(new_row as usize)
                    && let Some(&v) = r.get(new_col as usize)
                    && height >= v
                {
                    continue 'outer;
                }
            }

            total += height + 1;
        }
    }

    total
}
