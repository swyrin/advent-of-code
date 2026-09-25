pub use aoc_parse;
#[doc(hidden)]
pub use inventory;
pub use macros_impl::{AocInput, aoc, part, sample};

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct AocPart {
    pub name: &'static str,
    pub run: fn(&str),
}

inventory::collect!(AocPart);
