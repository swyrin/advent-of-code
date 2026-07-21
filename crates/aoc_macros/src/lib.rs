mod types;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

use types::submission::SubmissionArgs;

/// Part code macro.
/// Followed by a function that accepts `input_type` and `run_sample`.
///
/// To run actual tests (personalised input), set `PRIVATE` environment variable to a truthy value.
///
/// Defaults to running sample test ("0").
///
/// # Example
/// ```
/// use aoc_macros::aoc_submission;
/// use aoc_libraries::core::aoc_input::AocInput;
///
/// pub struct Ligma {
///     pub x: isize,
/// }
///
/// impl AocInput for Ligma {
///     fn from_raw_string(content: &str) -> Self {
///         Self { x: 42 }
///     }
/// }
///
/// #[aoc_submission(
///     input_type = crate::Ligma,
///     sample_in = "42",
///     sample_out = 2,
/// )]
/// fn test(input: Ligma, is_test: bool) -> isize {
///     2
/// }
/// ```
#[proc_macro_attribute]
pub fn aoc_submission(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as SubmissionArgs);
    let inner = parse_macro_input!(input as ItemFn);

    let inner_name = &inner.sig.ident;
    let input_type = &args.input_type;
    let sample_in = &args.sample_in;
    let sample_out = &args.sample_out;

    let test_name = syn::Ident::new(&format!("test_method_{}", inner_name), inner_name.span());
    let mod_name = syn::Ident::new(&format!("test_module_{}", inner_name), inner_name.span());
    let output_file_name =
        syn::LitStr::new(&format!("output_{}.txt", inner_name), inner_name.span());

    let expanded = quote! {
        #[allow(dead_code)]
        #[forbid(unsafe_code)]
        #inner

        #[cfg(test)]
        mod #mod_name {
            use std::fs;
            use aoc_libraries::utils::test_environment;
            use aoc_libraries::core::aoc_input::AocInput;
            use aoc_libraries::core::aoc_output::AocOutput;

            #[test]
            fn #test_name()
            where
                #input_type : AocInput,
            {
                let should_run_actual_test = test_environment::should_run_with_personalised_input();

                let input = match should_run_actual_test {
                    true => fs::read_to_string("input.txt").expect("Unable to read private input"),
                    false => String::from(#sample_in)
                };

                let input = #input_type::from_raw_string(&input);
                let output = crate::#inner_name(input);

                let test_result = match should_run_actual_test {
                    true => {
                        let output_private = format!("{}", output.answer);
                        fs::write(#output_file_name, output_private).expect("Unable to write output.");
                    },
                    false => {
                        let output_expected = AocOutput::from_number(#sample_out);

                        assert_eq!(
                            output,
                            output_expected
                        );
                    }
                };
            }
        }
    };

    expanded.into()
}
