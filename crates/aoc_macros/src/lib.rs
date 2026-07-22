mod types;

use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, parse_macro_input};

use types::submission::SubmissionArgs;

/// Part code macro.
///
/// To run actual tests (personalised input), set `PRIVATE` environment variable to a truthy value.
/// Defaults to running sample test ("0").
///
/// # Example
/// ```
/// use aoc_macros::aoc_submission;
///
/// pub struct Ligma {
///     pub x: isize,
/// }
///
/// impl std::str::FromStr for Ligma {
///     type Err = std::num::ParseIntError;
///
///     fn from_str(content: &str) -> Result<Self, Self::Err> {
///         Ok(Self { x: content.parse()? })
///     }
/// }
///
/// #[aoc_submission(
///     sample_in = "42",
///     sample_out = 2,
/// )]
/// fn test(input: Ligma) -> isize {
///     2
/// }
///
/// #[aoc_submission(ignore = "Requires manual inspection")]
/// fn visual_part(input: Ligma) -> isize {
///     input.x
/// }
/// ```
#[proc_macro_attribute]
pub fn aoc_submission(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as SubmissionArgs);
    let inner = parse_macro_input!(input as ItemFn);

    let inner_name = &inner.sig.ident;
    let input_type = match inner.sig.inputs.first() {
        Some(FnArg::Typed(argument)) => argument.ty.as_ref(),
        _ => {
            return syn::Error::new_spanned(
                &inner.sig,
                "`aoc_submission` requires an input argument",
            )
            .to_compile_error()
            .into();
        }
    };
    let test_name = syn::Ident::new(&format!("test_method_{}", inner_name), inner_name.span());
    let output_file_name =
        syn::LitStr::new(&format!("output_{}.txt", inner_name), inner_name.span());

    let generated_test = match args {
        SubmissionArgs::Sample {
            sample_in,
            sample_out,
        } => quote! {
            #[cfg(test)]
            #[test]
            fn #test_name()
            where
                #input_type: std::str::FromStr,
                <#input_type as std::str::FromStr>::Err: std::fmt::Display,
            {
                let should_run_actual_test = aoc_libraries::utils::test_environment::should_run_with_personalised_input();

                let raw_input = match should_run_actual_test {
                    true => std::fs::read_to_string("input.txt").expect("Unable to read private input"),
                    false => String::from(#sample_in)
                };

                let input = raw_input.parse::<#input_type>().unwrap_or_else(|error| {
                    panic!("Unable to parse input: {error}")
                });
                let output = #inner_name(input).to_string();

                if should_run_actual_test {
                    std::fs::write(#output_file_name, output).expect("Unable to write output.");
                } else {
                    let output_expected = (#sample_out).to_string();

                    assert_eq!(output, output_expected);
                }
            }
        },
        SubmissionArgs::Ignored { reason } => quote! {
            #[cfg(test)]
            #[test]
            #[ignore = #reason]
            fn #test_name() {}
        },
    };

    let expanded = quote! {
        #[allow(dead_code, clippy::let_and_return)]
        #[forbid(unsafe_code)]
        #inner

        #generated_test
    };

    expanded.into()
}
