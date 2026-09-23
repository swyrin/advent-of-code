use syn::parse::{Parse, ParseStream};
use syn::{ItemFn, Result};

pub struct AocInput {
    pub functions: Vec<ItemFn>,
}

impl Parse for AocInput {
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
