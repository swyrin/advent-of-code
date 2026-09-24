use proc_macro2::TokenStream;
use syn::Result;

/// Parsing syntax.
pub struct ParseSpec {
    /// The ... part inside #[parse(...)]
    pub expr: TokenStream,
}

impl ParseSpec {
    pub fn parse(attr: &syn::Attribute) -> Result<Self> {
        Ok(Self {
            expr: attr.parse_args::<TokenStream>()?,
        })
    }
}
