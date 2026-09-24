use syn::parse::{Parse, ParseStream};
use syn::{ItemFn, Result};

/// Representing a day in Advent of Code.
pub struct AdventOfCode {
    /// Every functions sat inside `aoc!`
    pub functions: Vec<ItemFn>,
}

impl Parse for AdventOfCode {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut functions = Vec::new();

        while !input.is_empty() {
            functions.push(input.parse::<ItemFn>()?);
        }

        Ok(Self {
            functions,
        })
    }
}
