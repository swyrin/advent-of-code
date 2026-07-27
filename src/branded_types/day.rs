use std::fmt;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Day(u8);

impl Default for Day {
    fn default() -> Self {
        Self(1)
    }
}

impl From<u8> for Day {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Day> for u8 {
    fn from(value: Day) -> Self {
        value.0
    }
}

impl Display for Day {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for Day {
    type Err = ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse().map(Self)
    }
}
