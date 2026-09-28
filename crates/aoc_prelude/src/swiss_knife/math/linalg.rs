/// Transpose 2D [`Vec<Vec<_>>`]
///
/// Source: https://stackoverflow.com/a/64499219
pub fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();

    (0..len).map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect::<Vec<T>>()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_array_transposition_square() {
        assert_eq!(transpose2(vec![vec![1, 0], vec![0, 1]]), [[1, 0], [0, 1]]);
    }

    #[test]
    fn test_array_transposition_non_square() {
        assert_eq!(transpose2(vec![vec![1, 2, 3], vec![4, 5, 6]]), [[1, 4], [2, 5], [3, 6]]);
    }
}
