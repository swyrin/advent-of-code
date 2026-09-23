use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ItemFn, Result, Type};

use crate::aoc::sample::{Sample, extract_samples};

pub struct Part {
    pub function: ItemFn,
    pub samples: Vec<Sample>,
}

pub fn extract_part(functions: &mut Vec<ItemFn>, name: &str) -> Result<Option<Part>> {
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

    let ty = (*reference.elem).clone();

    validate_unpacks(&arg.pat, &ty)?;

    Ok(ty)
}

fn validate_unpacks(pat: &syn::Pat, ty: &Type) -> Result<()> {
    let struct_pat = match pat {
        syn::Pat::Struct(pat) => pat,

        syn::Pat::Ident(binding) => match binding.subpat.as_ref() {
            Some((_, sub)) => match sub.as_ref() {
                syn::Pat::Struct(pat) => pat,

                _ => {
                    return Err(unpack_error(&binding.ident));
                },
            },

            None => {
                return Err(unpack_error(&binding.ident));
            },
        },

        _ => {
            return Err(unpack_error(pat));
        },
    };

    let ty_name = match ty {
        syn::Type::Path(path) => match path.path.segments.last() {
            Some(segment) => segment.ident.to_string(),

            None => {
                return Err(unpack_error(&struct_pat.path));
            },
        },

        _ => {
            return Err(unpack_error(&struct_pat.path));
        },
    };

    match struct_pat.path.segments.last() {
        Some(segment) if segment.ident == ty_name => Ok(()),

        _ => Err(Error::new_spanned(
            &struct_pat.path,
            "part function pattern must name the input type, e.g. `Input { foo }`",
        )),
    }
}

fn unpack_error(tokens: impl quote::ToTokens) -> Error {
    Error::new_spanned(
        tokens,
        "part functions must unpack the input struct, e.g. `Input { foo }: &Input`",
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
