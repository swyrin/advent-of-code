use aoc_macros::aoc_submission;
use aoc_utils::traits::generalised_output::UmiAteTheOutput;
use aoc_utils::traits::parsable_input::ParsableInput;

pub struct Ligma {
    pub x: isize,
}

impl ParsableInput for Ligma {
    fn from_raw_string(_: &str) -> Self {
        Self { x: 42 }
    }
}

#[aoc_submission(
    input_type = crate::Ligma,
    sample_in = "42",
    sample_out = 2,
)]
fn test(_: Ligma) -> UmiAteTheOutput {
    UmiAteTheOutput::from_number(2)
}
