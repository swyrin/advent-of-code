//! Crates that help me to conquer Advent of Code days.
//!
//! Everthing is re-exported as-is, except for
//! - Crates with dashes - of which will be replaced with underscores.
//! - Home-made crates - of which will be prefixed by `aoc_`

pub mod swiss_knife;

pub use counter;
pub use geo;
pub use good_lp;
pub use image;
pub use itertools;
pub use num;
pub use petgraph;
pub use range_set_blaze;
pub use swiss_knife as aoc_swiss_knife;
