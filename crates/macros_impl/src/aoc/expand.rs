use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, Result, parse_macro_input};

use crate::aoc::parser::AocInput;
use crate::aoc::part::{extract_part, generate_part, input_type_from_function};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AocInput);

    match expand_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn expand_impl(mut input: AocInput) -> Result<proc_macro2::TokenStream> {
    let part_one = extract_part(&mut input.functions, "part_one")?;

    let part_two = extract_part(&mut input.functions, "part_two")?;

    let input_type = match (&part_one, &part_two) {
        (Some(part), _) => input_type_from_function(&part.function)?,

        (_, Some(part)) => input_type_from_function(&part.function)?,

        (None, None) => {
            return Err(Error::new(
                proc_macro2::Span::call_site(),
                "expected `part_one` or `part_two`",
            ));
        },
    };

    let (part_one_code, part_one_run) = generate_part(part_one, &input_type, 1);

    let (part_two_code, part_two_run) = generate_part(part_two, &input_type, 2);

    let functions = &input.functions;

    Ok(quote! {
        #(#functions)*
        #part_one_code
        #part_two_code

        fn main() {
            let input_content =
                std::fs::read_to_string("input.txt")
                    .expect("No input.txt file");

            let input: #input_type =
                input_content
                    .parse()
                    .expect("Failed to parse input.txt");

            #part_one_run
            #part_two_run
        }
    })
}
