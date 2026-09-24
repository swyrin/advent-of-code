use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ItemFn, Type};

use crate::aoc::sample::{Sample, extract_samples};

/// Representing a part of an Advent of Code day.
pub struct Part {
    /// The function of which a part is associated with.  
    pub function: ItemFn,

    /// Its associated sample tests.
    ///
    /// Most Advent of Code days have just one, except some simulation problems.
    pub samples: Vec<Sample>,
}

/// Extract functions with the names: `part_one`, and `part_two`
///
/// Returns a tuple of `Option`<`Part`>s where:
///     - First item is `part_one`
///     - Second item is `part_two`
///
/// `None` is, obviously, the named ones does not exist.
pub fn extract_parts(functions: &[ItemFn]) -> syn::Result<(Option<Part>, Option<Part>)> {
    let part_one = functions.iter().filter(|func| func.sig.ident == "part_one").collect::<Vec<_>>();

    let p1_impl = if let Some(p1_fn) = part_one.first() {
        let mut p1_fn = (*p1_fn).clone().to_owned();
        let samples = extract_samples(&mut p1_fn.attrs)?;

        Some(Part {
            function: p1_fn,
            samples,
        })
    } else {
        None
    };

    let part_two = functions.iter().filter(|func| func.sig.ident == "part_two").collect::<Vec<_>>();

    let p2_impl = if let Some(p2_fn) = part_two.first() {
        let mut p2_fn = (*p2_fn).to_owned();
        let samples = extract_samples(&mut p2_fn.attrs)?;

        Some(Part {
            function: p2_fn,
            samples,
        })
    } else {
        None
    };

    Ok((p1_impl, p2_impl))
}

/// Extract the type of the input struct from the input function.
///
/// Usually will be used in conjunction with test "generators".
pub fn get_input_type_from_function(function: &ItemFn) -> syn::Result<Type> {
    if function.sig.inputs.len() == 1
        && let Some(first_arg) = function.sig.inputs.first()
        && let syn::FnArg::Typed(arg) = first_arg
        && let syn::Type::Reference(reference) = arg.ty.as_ref()
    {
        Ok((*reference.elem).clone())
    } else {
        Err(Error::new_spanned(
            &function.sig,
            "function must be ONLY ONE input parameter with a reference type.",
        ))
    }
}

/// Generate part function.
///
/// If everything is done correctly, this function simply passes
/// what you have written into `part_one` and/or `part_two` inside `aoc!`
///
/// Returns two thing:
///     - Part definition + tests
///     - Glue code to put into `main`
pub fn generate_part(part: Option<Part>, input_type: Option<&Type>) -> (TokenStream, TokenStream) {
    let (Some(part), Some(input_type)) = (part, input_type) else {
        return (quote! {}, quote! {});
    };

    let Part {
        function,
        samples,
    } = part;

    let function_name = &function.sig.ident;
    let function_name_string = function_name.to_string();

    let tests = samples
        .into_iter()
        .enumerate()
        .map(|(index, sample)| {
            let sample_input = sample.input;
            let expected = sample.expected;
            let part_number = if function_name == "part_one" { 1u8 } else { 2 };

            let test_name = syn::Ident::new(
                &format!("part_{part_number}_sample_{index}"),
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
        })
        .collect::<Vec<_>>();

    let run = quote! {
        println!(
            "Result of {}: {}",
            #function_name_string,
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
