/// Read: input with type safety.
pub trait ParsableInput {
    /// Read input from a raw `&str`.
    ///
    /// Usually, you should be using the `aoc_macro::aoc_submission`
    /// macro instead of this.
    fn from_raw_string(content: &str) -> Self;
}
