use proc_macro::TokenStream;

mod aoc;
mod input;

#[proc_macro]
pub fn aoc(input: TokenStream) -> TokenStream {
    aoc::expand::expand(input).into()
}

#[proc_macro_derive(AocInput, attributes(parse))]
pub fn aoc_input(input: TokenStream) -> TokenStream {
    input::expand::expand(input)
}
