use proc_macro::TokenStream;

mod aoc;
mod input;

/// The entry point for everything.
///
/// Calling `aoc!();` alone is enough.
#[proc_macro]
pub fn aoc(input: TokenStream) -> TokenStream {
    aoc::expand::expand(input.into()).unwrap_or_else(|error| error.into_compile_error()).into()
}

/// Registers a solution function and generates its sample tests.
///
/// A part must accept one shared reference and return a value that implements `Display`.
#[proc_macro_attribute]
pub fn part(attribute: TokenStream, item: TokenStream) -> TokenStream {
    aoc::part::expand(attribute.into(), item.into())
        .unwrap_or_else(|error| error.into_compile_error())
        .into()
}

/// Place it after `#[part]` as `#[sample(input = "...", expected = "...")]`.
#[proc_macro_attribute]
pub fn sample(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let attribute = proc_macro2::TokenStream::from(attribute);
    let item = proc_macro2::TokenStream::from(item);

    let output: syn::Result<proc_macro2::TokenStream> = (|| {
        let mut function: syn::ItemFn = syn::parse2(item)?;
        let marker = function
            .attrs
            .iter()
            .position(|attr| attr.path().is_ident(crate::aoc::sample::SAMPLE_MARKER))
            .ok_or_else(|| {
                syn::Error::new_spanned(attribute, "#[sample] is only valid after #[part]")
            })?;

        function.attrs.remove(marker);

        Ok(quote::quote! { #function })
    })();

    output.unwrap_or_else(|error| error.into_compile_error()).into()
}

/// If you don't want manual labor of [`aoc_parse`] and [`std::str::FromStr`],
/// please use this inside `#[derive]`.
#[proc_macro_derive(AocInput, attributes(parse))]
pub fn aoc_input(input: TokenStream) -> TokenStream {
    input::expand::expand(input).into()
}
