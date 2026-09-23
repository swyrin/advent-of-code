use quote::quote;

use crate::aoc::parser::AocInput;
use crate::aoc::part::{extract_part, generate_part, input_type_from_function};

pub fn expand(input: impl Into<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    let raw = input.into();

    // Empty input means no parts.
    let parsed: syn::Result<AocInput> = if raw.is_empty() {
        Ok(AocInput {
            functions: Vec::new(),
        })
    } else {
        syn::parse2(raw.clone())
    };

    let output: syn::Result<proc_macro2::TokenStream> = parsed.and_then(|mut input| {
        let part_one = extract_part(&mut input.functions, "part_one")?;
        let part_two = extract_part(&mut input.functions, "part_two")?;
        let helpers = &input.functions;

        let input_type = part_one
            .as_ref()
            .or(part_two.as_ref())
            .map(|part| input_type_from_function(&part.function))
            .transpose()?;

        let (part_one_code, part_one_run) =
            generate_part(part_one, input_type.as_ref(), 1, "part_one");
        let (part_two_code, part_two_run) =
            generate_part(part_two, input_type.as_ref(), 2, "part_two");

        let parse_input = input_type.as_ref().map(|ty| {
            quote! {
                let input: #ty = _input_content.parse().expect("Failed to parse input.txt");
            }
        });

        Ok(quote! {
            #(#helpers)*
            #part_one_code
            #part_two_code

            fn main() {
                let _input_content =
                    std::fs::read_to_string("input.txt").expect("No input.txt file");

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

                fn main() {
                    let _input_content =
                        std::fs::read_to_string("input.txt").expect("No input.txt file");
                }

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
    fn valid_parts_keep_both_tests() {
        let out = expand(quote! {
            #[sample(input = "1", expected = "1")]
            fn part_one(Input { n }: &Input) -> impl std::fmt::Display {
                n
            }

            #[sample(input = "2", expected = "2")]
            fn part_two(Input { n }: &Input) -> impl std::fmt::Display {
                n
            }
        })
        .to_string();

        assert!(out.contains("part_1_sample_0"));
        assert!(out.contains("part_2_sample_0"));
        assert!(!out.contains("compile_error"));
        assert!(!out.contains("todo"));
    }

    #[test]
    fn bare_input_without_unpack_works() {
        let out = expand(quote! {
            #[sample(input = "1", expected = "1")]
            fn part_one(input: &Input) -> impl std::fmt::Display {
                input.n
            }

            #[sample(input = "2", expected = "2")]
            fn part_two(input: &Input) -> impl std::fmt::Display {
                input.n
            }
        })
        .to_string();

        assert!(out.contains("part_1_sample_0"));
        assert!(out.contains("part_2_sample_0"));
        assert!(!out.contains("compile_error"));
        assert!(!out.contains("todo"));
    }

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

        assert!(out.contains("compile_error"));
        assert!(!out.contains("sample_0"));
    }

    #[test]
    fn broken_syntax_keeps_both_functions_verbatim() {
        let out = expand(quote! {
            #[sample(input = "1", expected = "1")]
            fn part_one(Input { n }: &Input) -> impl std::fmt::Display {
                let x = ;
                n
            }

            #[sample(input = "2", expected = "2")]
            fn part_two(Input { n }: &Input) -> impl std::fmt::Display {
                n
            }
        })
        .to_string();

        assert!(out.contains("fn part_one"));
        assert!(out.contains("fn part_two"));
        assert!(out.contains("sample (input"));
        assert!(!out.contains("sample_0"));
        assert!(out.contains("compile_error"));
    }

    #[test]
    fn missing_part_two_becomes_todo() {
        let out = expand(quote! {
            #[sample(input = "1", expected = "1")]
            fn part_one(Input { n }: &Input) -> impl std::fmt::Display {
                n
            }
        })
        .to_string();

        assert!(out.contains("part_1_sample_0"));
        assert!(out.contains("fn part_two"));
        assert!(out.contains("todo"));
        assert!(!out.contains("part_two (& input)"));
        assert!(!out.contains("compile_error"));
    }

    #[test]
    fn missing_both_parts_become_todo() {
        let out = expand(quote! {
            fn helper() {}
        })
        .to_string();

        assert!(out.contains("fn part_one"));
        assert!(out.contains("fn part_two"));
        assert!(out.contains("todo"));
        assert!(!out.contains("(& input)"));
        assert!(out.contains("read_to_string"));
        assert!(!out.contains("compile_error"));
    }

    #[test]
    fn empty_input_becomes_todo() {
        let out = expand(quote! {}).to_string();

        assert!(out.contains("fn part_one"));
        assert!(out.contains("fn part_two"));
        assert!(out.contains("todo"));
        assert!(!out.contains("(& input)"));
        assert!(!out.contains("compile_error"));
    }
}
