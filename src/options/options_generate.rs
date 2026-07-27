use chrono::Local;
use chrono::prelude::*;
use clap::Args;

#[derive(Args)]
pub struct GenerateOptions {
    /// The year. Default to this year.
    #[arg(short, long, default_value_t = Local::now().year() as usize)]
    year: usize,

    /// The day. Default to day 1.
    #[arg(short, long, default_value_t = 1)]
    day: usize,

    /// Whether to overwrite.
    #[arg(long, default_value_t = false)]
    overwrite: bool,
}

impl GenerateOptions {
    pub fn new(year: usize, day: usize, overwrite: bool) -> Self {
        Self {
            year,
            day,
            overwrite,
        }
    }

    pub fn get_day(&self) -> usize {
        self.day
    }

    pub fn get_year(&self) -> usize {
        self.year
    }

    pub fn get_overwrite(&self) -> bool {
        self.overwrite
    }
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            year: Local::now().year() as usize,
            day: 1,
            overwrite: false,
        }
    }
}
