/// Return all adjacents of a position.
///
/// Direct adjacents only, unless `include_diagonals` is `true`.
///
/// Set `valid_only` to `true` if you don't to have `None` in either places,
/// which signify an overflow.
pub fn adjacent_neighbors(
    r: usize,
    c: usize,
    include_diagonals: bool,
    valid_only: bool,
) -> Vec<(Option<usize>, Option<usize>)> {
    let mut output = vec![
        (Some(r), c.checked_sub(1)), // left
        (r.checked_add(1), Some(c)), // bot
        (Some(r), c.checked_add(1)), // right
        (r.checked_sub(1), Some(c)), // top
    ];

    if include_diagonals {
        output.push((r.checked_sub(1), c.checked_sub(1))); // top-left
        output.push((r.checked_add(1), c.checked_sub(1))); // bot-left
        output.push((r.checked_add(1), c.checked_add(1))); // bot-right
        output.push((r.checked_sub(1), c.checked_add(1))); // top-right
    }

    if valid_only {
        output.into_iter().filter(|(r, c)| r.is_some() && c.is_some()).collect()
    } else {
        output
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn direct_neighbors() {
        assert_eq!(adjacent_neighbors(5, 5, false, false), vec![
            (Some(5), Some(4)), // left
            (Some(6), Some(5)), // bottom
            (Some(5), Some(6)), // right
            (Some(4), Some(5)), // top
        ]);
    }

    #[test]
    fn direct_and_diagonal_neighbors() {
        assert_eq!(adjacent_neighbors(5, 5, true, false), vec![
            (Some(5), Some(4)), // left
            (Some(6), Some(5)), // bottom
            (Some(5), Some(6)), // right
            (Some(4), Some(5)), // top
            (Some(4), Some(4)), // top-left
            (Some(6), Some(4)), // bottom-left
            (Some(6), Some(6)), // bottom-right
            (Some(4), Some(6)), // top-right
        ]);
    }

    #[test]
    fn top_left_filters_invalid_neighbors() {
        assert_eq!(adjacent_neighbors(0, 0, false, true), vec![
            (Some(1), Some(0)), // bottom
            (Some(0), Some(1)), // right
        ]);
    }

    #[test]
    fn top_left_filters_invalid_diagonal_neighbors() {
        assert_eq!(adjacent_neighbors(0, 0, true, true), vec![
            (Some(1), Some(0)), // bottom
            (Some(0), Some(1)), // right
            (Some(1), Some(1)), // bottom-right
        ]);
    }

    #[test]
    fn preserves_invalid_neighbors_when_valid_only_is_false() {
        assert_eq!(adjacent_neighbors(0, 0, false, false), vec![
            (Some(0), None),    // left
            (Some(1), Some(0)), // bottom
            (Some(0), Some(1)), // right
            (None, Some(0)),    // top
        ]);
    }

    #[test]
    fn top_left_corner_with_diagonals() {
        assert_eq!(adjacent_neighbors(0, 0, true, false), vec![
            (Some(0), None),    // left
            (Some(1), Some(0)), // bottom
            (Some(0), Some(1)), // right
            (None, Some(0)),    // top
            (None, None),       // top-left
            (Some(1), None),    // bottom-left
            (Some(1), Some(1)), // bottom-right
            (None, Some(1)),    // top-right
        ]);
    }

    #[test]
    fn handles_usize_max_boundary() {
        let max = usize::MAX;

        assert_eq!(adjacent_neighbors(max, max, false, false), vec![
            (Some(max), Some(max - 1)), // left
            (None, Some(max)),          // bottom overflow
            (Some(max), None),          // right overflow
            (Some(max - 1), Some(max)), // top
        ]);
    }

    #[test]
    fn handles_usize_max_boundary_with_diagonals() {
        let max = usize::MAX;

        assert_eq!(adjacent_neighbors(max, max, true, false), vec![
            (Some(max), Some(max - 1)),
            (None, Some(max)),
            (Some(max), None),
            (Some(max - 1), Some(max)),
            (Some(max - 1), Some(max - 1)),
            (None, Some(max - 1)),
            (None, None),
            (Some(max - 1), None),
        ]);
    }

    #[test]
    fn valid_only_removes_all_overflowing_neighbors_at_bottom_right() {
        let max = usize::MAX;

        assert_eq!(adjacent_neighbors(max, max, true, true), vec![
            (Some(max), Some(max - 1)),
            (Some(max - 1), Some(max)),
            (Some(max - 1), Some(max - 1)),
        ]);
    }
}
