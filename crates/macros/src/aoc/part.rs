use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ItemFn, Result, Type};

use crate::aoc::sample::{Sample, extract_sample};

pub struct Part {
    pub function: ItemFn,
    pub sample: Option<Sample>,
}

pub fn extract_part(functions: &mut Vec<ItemFn>, name: &str) -> Result<Option<Part>> {
    let Some(index) = functions.iter().position(|function| function.sig.ident == name) else {
        return Ok(None);
    };

    let mut function = functions.remove(index);
    let sample = extract_sample(&mut function.attrs)?;

    Ok(Some(Part {
        function,
        sample,
    }))
}

pub fn input_type_from_function(function: &ItemFn) -> Result<Type> {
    let Some(first_arg) = function.sig.inputs.first() else {
        return Err(Error::new_spanned(
            &function.sig,
            "part function must have an input parameter",
        ));
    };

    let syn::FnArg::Typed(arg) = first_arg else {
        return Err(Error::new_spanned(
            first_arg,
            "part function must have a typed input parameter",
        ));
    };

    let syn::Type::Reference(reference) = arg.ty.as_ref() else {
        return Err(Error::new_spanned(
            &arg.ty,
            "part function input must be a reference, e.g. `&Input`",
        ));
    };

    Ok((*reference.elem).clone())
}

fn generate_sample_test(
    function_name: &syn::Ident,
    sample: Sample,
    input_type: &Type,
    part_number: u8,
) -> TokenStream {
    let sample_input = sample.input;
    let expected = sample.expected;

    let test_name =
        syn::Ident::new(&format!("part_{part_number}_sample"), proc_macro2::Span::call_site());

    quote! {
        #[test]
        fn #test_name() {
            let input: #input_type =
                #sample_input
                    .parse()
                    .expect(
                        concat!(
                            "Failed to parse sample input for part ",
                            #part_number
                        )
                    );

            let actual = #function_name(&input)
                .expect(
                    concat!(
                        "Part ",
                        #part_number,
                        " returned an error on sample input"
                    )
                );

            assert_eq!(
                actual.to_string(),
                #expected.to_string(),
                "Part {} sample failed",
                #part_number
            );
        }
    }
}

pub fn generate_part(
    part: Option<Part>,
    input_type: &Type,
    part_number: u8,
) -> (TokenStream, TokenStream) {
    let Some(part) = part else {
        return (quote! {}, quote! {});
    };

    let Part {
        function,
        sample,
    } = part;

    let function_name = &function.sig.ident;

    let test =
        sample.map(|sample| generate_sample_test(function_name, sample, input_type, part_number));

    let run = quote! {
        match #function_name(&input) {
            Ok(answer) => {
                println!(
                    "Result of part {}: {}",
                    #part_number,
                    answer
                );
            }

            Err(_) => {
                println!(
                    "Part {} fails.",
                    #part_number
                );
            }
        }
    };

    (
        quote! {
            #[forbid(unsafe_code)]
            #function
            #test
        },
        run,
    )
}
