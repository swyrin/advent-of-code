mod types;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

use types::submission::SubmissionArgs;

/// Part code macro.
/// Followed by a function that accepts `input_type` and `run_sample`.
///
/// # Example
/// ```
/// use aoc_macros::aoc_submission;
/// use aoc_utils::traits::parsable_input::ParsableInput;
///
/// pub struct Ligma {
///     pub x: isize,
/// }
///
/// impl ParsableInput for Ligma {
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

    let expanded = quote! {
        #[allow(dead_code)]
        #[forbid(unsafe_code)]
        #inner

        #[cfg(test)]
        mod #mod_name {
            use aoc_utils::traits::parsable_input::ParsableInput;
            use aoc_utils::traits::generalised_output::UmiAteTheOutput;

            #[test]
            fn #test_name()
            where
                #input_type : ParsableInput,
            {
                let input = <#input_type as ParsableInput>::from_raw_string(#sample_in);

                let output_expected = UmiAteTheOutput::from_number(#sample_out);
                let output_reality = crate::#inner_name(input);

                assert_eq!(
                    output_reality,
                    output_expected
                );
            }
        }
    };

    expanded.into()
}
