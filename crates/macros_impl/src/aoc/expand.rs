use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Error;

pub fn expand(input: TokenStream) -> syn::Result<TokenStream> {
    if !input.is_empty() {
        return Err(Error::new(Span::call_site(), "annotate part functions with #[part]"));
    }

    Ok(quote! {
        fn main() {
            let mut parts = ::macros::inventory::iter::<::macros::AocPart>()
                .copied()
                .collect::<Vec<_>>();

            if parts.is_empty() {
                return;
            }

            parts.sort_unstable_by_key(|part| part.name);

            let content = ::std::fs::read_to_string("input.txt").expect("No input.txt file");

            for part in parts {
                (part.run)(&content);
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn properly_generates_registry_runner() {
        let output = expand(quote!()).unwrap().to_string();

        assert!(output.contains("fn main"));
        assert!(output.contains("inventory :: iter"));
    }

    #[test]
    fn nonempty_input_errors() {
        let error = expand(quote! {
            fn helper() {}
        })
        .unwrap_err();

        assert!(error.to_string().contains("aoc! does not accept input"));
    }
}
