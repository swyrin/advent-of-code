use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Error, Expr, LitStr, Result, parse_quote};

pub(crate) const SAMPLE_MARKER: &str = "__aoc_part_sample";

/// Represent a sample test of an Advent of Code part.
pub struct Sample {
    /// A literal `&str` of input.
    pub input: LitStr,

    /// A literal `&str` of expected output.
    pub expected: Expr,
}

impl Parse for Sample {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut sample_input = None;
        let mut expected = None;

        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match name.to_string().as_str() {
                "input" => {
                    if sample_input.is_some() {
                        return Err(Error::new(name.span(), "duplicate `input`"));
                    }

                    sample_input = Some(input.parse::<LitStr>()?);
                },

                "expected" => {
                    if expected.is_some() {
                        return Err(Error::new(name.span(), "duplicate `expected`"));
                    }

                    expected = Some(input.parse::<Expr>()?);
                },

                _ => {
                    return Err(Error::new(name.span(), "expected `input` or `expected`"));
                },
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self {
            input: sample_input
                .ok_or_else(|| Error::new(Span::call_site(), "missing `input = ...`"))?,

            expected: expected
                .ok_or_else(|| Error::new(Span::call_site(), "missing `expected = ...`"))?,
        })
    }
}

pub fn mark_for_expansion(attrs: &mut Vec<Attribute>) {
    let original = std::mem::take(attrs);
    let marker = syn::Ident::new(SAMPLE_MARKER, proc_macro2::Span::call_site());

    *attrs = original
        .into_iter()
        .flat_map(|attr| {
            if attr.path().is_ident("sample") {
                vec![attr, parse_quote!(#[#marker])]
            } else {
                vec![attr]
            }
        })
        .collect();
}

/// Collect sample tests.
pub fn extract_samples(attrs: &[Attribute]) -> Result<Vec<Sample>> {
    let mut samples = Vec::new();

    for attr in attrs.iter() {
        if attr.path().is_ident("sample") {
            samples.push(attr.parse_args::<Sample>()?);
        }
    }

    Ok(samples)
}
