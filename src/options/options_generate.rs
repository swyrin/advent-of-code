use crate::branded_types::day::Day;
use crate::branded_types::year::Year;
use clap::Args;

#[derive(Args, Default)]
pub struct GenerateOptions {
    /// The year. Default to this year.
    #[arg(short, long, default_value_t)]
    year: Year,

    /// The day. Default to day 1.
    #[arg(short, long, default_value_t)]
    day: Day,

    /// Whether to overwrite.
    #[arg(long)]
    overwrite: bool,
}

impl GenerateOptions {
    #[cfg(test)]
    pub fn new(year: u16, day: u8, overwrite: bool) -> Self {
        Self {
            year: Year::from(year),
            day: Day::from(day),
            overwrite,
        }
    }

    pub fn get_day(&self) -> u8 {
        self.day.into()
    }

    pub fn get_year(&self) -> u16 {
        self.year.into()
    }

    pub fn get_overwrite(&self) -> bool {
        self.overwrite
    }
}
