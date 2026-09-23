use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Ident, ItemFn, Type};

use crate::aoc::sample::{Sample, extract_samples};

pub struct Part {
    pub function: ItemFn,
    pub samples: Vec<Sample>,
}

pub fn extract_part(functions: &mut Vec<ItemFn>, name: &str) -> syn::Result<Option<Part>> {
    let Some(index) = functions.iter().position(|function| function.sig.ident == name) else {
        return Ok(None);
    };

    let mut function = functions.remove(index);
    let samples = extract_samples(&mut function.attrs)?;

    Ok(Some(Part {
        function,
        samples,
    }))
}

pub fn input_type_from_function(function: &ItemFn) -> syn::Result<Type> {
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

    let ty = (*reference.elem).clone();

    Ok(ty)
}

pub fn generate_part(
    part: Option<Part>,
    input_type: Option<&Type>,
    part_number: u8,
    name: &str,
) -> (TokenStream, TokenStream) {
    let (Some(part), Some(input_type)) = (part, input_type) else {
        let ident = Ident::new(name, proc_macro2::Span::call_site());
        let message = format!("aoc!: `{name}` is not defined");

        return (
            quote! {
                #[forbid(unsafe_code)]
                fn #ident() -> impl std::fmt::Display {
                    todo!(#message)
                }
            },
            quote! {},
        );
    };

    let Part {
        function,
        samples,
    } = part;

    let function_name = &function.sig.ident;

    let tests = samples
        .into_iter()
        .enumerate()
        .map(|(index, sample)| {
            generate_sample_test(function_name, sample, input_type, part_number, index)
        })
        .collect::<Vec<_>>();

    let run = quote! {
        println!(
            "Result of part {}: {}",
            #part_number,
            #function_name(&input)
        );
    };

    (
        quote! {
            #[forbid(unsafe_code)]
            #function
            #(#tests)*
        },
        run,
    )
}

fn generate_sample_test(
    function_name: &syn::Ident,
    sample: Sample,
    input_type: &Type,
    part_number: u8,
    sample_index: usize,
) -> TokenStream {
    let sample_input = sample.input;
    let expected = sample.expected;

    let test_name = syn::Ident::new(
        &format!("part_{part_number}_sample_{sample_index}"),
        proc_macro2::Span::call_site(),
    );

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

            let actual = #function_name(&input);

            assert_eq!(
                actual.to_string(),
                #expected.to_string(),
                "Part {} sample failed",
                #part_number
            );
        }
    }
}
