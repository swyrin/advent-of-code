use quote::quote;

use crate::aoc::parser::AdventOfCode;
use crate::aoc::part::{extract_parts, generate_part, get_input_type_from_function};

pub fn expand(input: impl Into<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    let raw = input.into();
    let parsed: syn::Result<AdventOfCode> = syn::parse2(raw.clone());

    let output: syn::Result<proc_macro2::TokenStream> = parsed.and_then(|input| {
        let (part_one, part_two) = extract_parts(&input.functions)?;

        // like... you have to do part one right?
        let input_type = part_one
            .as_ref()
            .or(part_two.as_ref())
            .map(|part| get_input_type_from_function(&part.function))
            .transpose()?;

        let (part_one_code, part_one_run) = generate_part(part_one, input_type.as_ref());
        let (part_two_code, part_two_run) = generate_part(part_two, input_type.as_ref());

        let parse_input = input_type.as_ref().map(|ty| {
            quote! {
                let _input_content = std::fs::read_to_string("input.txt").expect("No input.txt file");
                let input: #ty = _input_content.parse().expect("Failed to parse input.txt");
            }
        });

        Ok(quote! {
            #part_one_code
            #part_two_code

            fn main() {
                #parse_input

                #part_one_run
                #part_two_run
            }
        })
    });

    match output {
        Ok(tokens) => tokens,
        Err(error) => {
            let compile_error = error.into_compile_error();

            quote! {
                #raw
                #compile_error
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn extra_param_errors() {
        let out = expand(quote! {
            #[sample(input = "1", expected = "1")]
            fn part_one(input: &Input, extra: u32) -> impl std::fmt::Display {
                input.n
            }

            #[sample(input = "2", expected = "2")]
            fn part_two(input: &Input) -> impl std::fmt::Display {
                input.n
            }
        })
        .to_string();

        assert!(out.contains("compile_error !"));
    }

    #[test]
    fn with_mixed_multiple_samples() {
        let out = expand(quote! {
            #[sample(input = "1", expected = "1")]
            #[sample(input = "10", expected = "10")]
            fn part_one(Input { n }: &Input) -> impl std::fmt::Display {
                n
            }

            #[sample(input = "1", expected = "1")]
            fn part_two(Input { n }: &Input) -> impl std::fmt::Display {
                n
            }
        })
        .to_string();

        assert!(out.contains("part_1_sample_0"));
        assert!(out.contains("part_1_sample_1"));

        assert!(out.contains("part_2_sample_0"));
    }
}
