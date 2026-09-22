/// Transpose 2D [`Vec<Vec<_>>`]
///
/// Source: https://stackoverflow.com/a/64499219
pub fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();

    (0..len).map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect::<Vec<T>>()).collect()
}
