use chrono::{Datelike, Local};
use std::fmt;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Year(u16);

impl Default for Year {
    fn default() -> Self {
        Self(u16::try_from(Local::now().year()).expect("Are we in year 60k?"))
    }
}

impl From<u16> for Year {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Year> for u16 {
    fn from(value: Year) -> Self {
        value.0
    }
}

impl Display for Year {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for Year {
    type Err = ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse().map(Self)
    }
}
