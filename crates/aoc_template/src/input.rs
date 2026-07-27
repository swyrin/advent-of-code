pub struct Input {
    pub everything: usize,
}

impl std::str::FromStr for Input {
    type Err = aoc_libraries::aoc_parse::ParseError;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Self { everything: 42 })
    }
}
