use syn::parse::{Parse, ParseStream};
use syn::{Error, ItemFn, Result};

pub struct AocInput {
    pub functions: Vec<ItemFn>,
}

impl Parse for AocInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut functions = Vec::new();

        while !input.is_empty() {
            functions.push(input.parse::<ItemFn>()?);
        }

        if functions.is_empty() {
            return Err(Error::new(input.span(), "expected at least one part function"));
        }

        Ok(Self {
            functions,
        })
    }
}
