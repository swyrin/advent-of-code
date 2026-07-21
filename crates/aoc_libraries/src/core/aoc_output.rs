use num::bigint::ToBigUint;
use num::{BigUint, Num};

#[derive(Debug, PartialEq)]
pub struct AocOutput {
    /// The underlying answer.
    pub answer: BigUint,
}

impl AocOutput {
    pub fn from_number<T>(num: T) -> Self
    where
        T: Num + ToBigUint,
    {
        Self {
            answer: num.to_biguint().unwrap(),
        }
    }
}
