use num::bigint::ToBigUint;
use num::{BigUint, Num};

/// The answer seems a little bit cute today?
#[derive(Debug, PartialEq)]
pub struct UmiAteTheOutput {
    /// The underlying answer.
    pub answer: BigUint,
}

impl UmiAteTheOutput {
    pub fn from_number<T>(num: T) -> Self
    where
        T: Num + ToBigUint,
    {
        Self {
            answer: num.to_biguint().unwrap(),
        }
    }
}
