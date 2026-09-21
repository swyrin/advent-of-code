use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Result, parse_macro_input};

use super::parser::ParseSpec;

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn expand_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let struct_name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,

            _ => {
                return Err(Error::new_spanned(
                    struct_name,
                    "AocInput requires a struct with named fields",
                ));
            },
        },

        _ => {
            return Err(Error::new_spanned(
                struct_name,
                "AocInput can only be derived for structs",
            ));
        },
    };

    if fields.len() != 1 {
        return Err(Error::new_spanned(fields, "AocInput currently requires exactly one field"));
    }

    let field = fields.first().unwrap();

    let field_name = field.ident.as_ref().unwrap();

    let parse_attr = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("parse"))
        .ok_or_else(|| Error::new_spanned(field, "missing `#[parse(...)]` attribute"))?;

    let spec = ParseSpec::parse(parse_attr)?;

    let definitions = spec.definitions.iter().map(|definition| {
        let name = &definition.name;
        let expr = &definition.expr;

        quote! {
            let #name = aoc_parse::parser!(#expr);
        }
    });

    let final_expr = &spec.final_expr;

    Ok(quote! {
        impl std::str::FromStr for #struct_name {
            type Err = aoc_parse::ParseError;

            fn from_str(
                s: &str,
            ) -> Result<Self, Self::Err> {
                #(#definitions)*

                let __aoc_final =
                    aoc_parse::parser!(#final_expr);

                let #field_name =
                    __aoc_final.parse(s)?;

                Ok(Self {
                    #field_name,
                })
            }
        }
    })
}
