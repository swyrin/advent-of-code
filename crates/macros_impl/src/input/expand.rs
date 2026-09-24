use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Result};

use super::parser::ParseSpec;

pub fn expand(input: impl Into<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    let parsed: Result<DeriveInput> = syn::parse2(input.into());

    let output: Result<proc_macro2::TokenStream> = parsed.and_then(|input| {
        let struct_name = input.ident.clone();

        if let Some(attr) = input.attrs.iter().find(|attr| attr.path().is_ident("parse")) {
            return Err(Error::new_spanned(
                attr,
                "`#[parse(...)]` belongs on the fields of an `AocInput` struct, not on the struct",
            ));
        }

        let fields = match &input.data {
            Data::Struct(data) => match &data.fields {
                Fields::Named(fields) => fields.named.iter().cloned().collect::<Vec<_>>(),

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

        let mut specs = Vec::new();

        for field in &fields {
            let name = field.ident.as_ref().unwrap().clone();

            let parse_attr = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("parse"))
                .ok_or_else(|| Error::new_spanned(field, "missing `#[parse(...)]` attribute"))?;

            specs.push((name, ParseSpec::parse(parse_attr)?));
        }

        let bindings = specs.iter().map(|(name, _)| name.clone()).collect::<Vec<_>>();

        let constructor = quote!(Ok(Self {
            #(#bindings),*
        }));

        let from_str_body = if specs.is_empty() {
            quote! {
                let _ = __aoc_input;
                Ok(Self {})
            }
        } else {
            let parsers = specs.iter().map(|(name, spec)| {
                let expr = &spec.expr;

                quote! {
                    let #name = macros::aoc_parse::parser!(#expr);
                }
            });

            let labels: Vec<syn::Ident> =
                (0..specs.len()).map(|index| quote::format_ident!("__aoc_f{}", index)).collect();

            quote! {
                use macros::aoc_parse::Parser;
                use macros::aoc_parse::prelude::*;

                #(#parsers)*

                let ( #(#bindings,)* ) = macros::aoc_parse::parser!(
                    #(#labels:#bindings)*
                    => ( #(#labels,)* )
                )
                .parse(__aoc_input)?;

                #constructor
            }
        };

        Ok(quote! {
            impl std::str::FromStr for #struct_name {
                type Err = macros::aoc_parse::ParseError;

                /// How many underscores do I need?
                fn from_str(
                    __aoc_input: &str,
                ) -> Result<Self, Self::Err> {
                    #from_str_body
                }
            }
        })
    });

    match output {
        Ok(tokens) => tokens,
        Err(error) => error.into_compile_error(),
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn empty_struct_parses_nothing() {
        let out = expand(quote! {
            struct Input {}
        })
        .to_string();

        assert!(out.contains("Self { }"));
        assert!(out.contains("__aoc_input"));
    }

    #[test]
    fn single_field_parses_whole_input() {
        let out = expand(quote! {
            struct Input {
                #[parse(line(u32))]
                number: u32,
            }
        })
        .to_string();

        assert!(out.contains("number"));
        assert!(out.contains("parser !"));
        assert!(out.contains("__aoc_f0"));
    }

    #[test]
    fn multi_field_gathers_through_mothership() {
        let out = expand(quote! {
            struct Input {
                #[parse(section(line(u32)))]
                first: u32,
                #[parse(section(line(u64)))]
                second: u64,
            }
        })
        .to_string();

        assert!(out.contains("let first ="));
        assert!(out.contains("let second ="));
        assert!(out.contains("__aoc_f0 : first"));
        assert!(out.contains("__aoc_f1 : second"));
    }

    #[test]
    fn rules_pass_through_verbatim() {
        let out = expand(quote! {
            struct Input {
                #[parse(
                    rule light: bool = {
                        "." => false,
                        "#" => true,
                    };
                    lines(light+)
                )]
                rows: Vec<Vec<bool>>,
            }
        })
        .to_string();

        assert!(out.contains("rule"));
    }

    #[test]
    fn recursive_rules_pass_through() {
        let out = expand(quote! {
            struct Input {
                #[parse(
                    rule formation: Formation = {
                        s:alpha => Formation::Elf(s),
                        v:stack => Formation::Stack(v),
                    };
                    rule stack: Vec<Formation> = '(' v:formation+ ')' => v;
                    lines(formation+)
                )]
                formations: Vec<Vec<Formation>>,
            }
        })
        .to_string();

        assert!(out.contains("rule formation"));
        assert!(out.contains("rule stack"));
        assert!(out.contains("formations"));
        assert!(out.contains("__aoc_f0"));
    }

    #[test]
    fn top_level_mapper_passes_through() {
        let out = expand(quote! {
            struct Input {
                #[parse(rows:lines(string(any_char+)) => rows.len())]
                width: usize,
            }
        })
        .to_string();

        assert!(out.contains("=>"));
    }

    #[test]
    fn empty_parse_is_parser_problem() {
        let out = expand(quote! {
            struct Input {
                #[parse()]
                number: u32,
            }
        })
        .to_string();

        assert!(out.contains("parser !"));
    }

    #[test]
    fn parse_on_struct_errors() {
        let out = expand(quote! {
            #[parse(line(u32))]
            struct Input {
                #[parse(line(u32))]
                number: u32,
            }
        })
        .to_string();

        assert!(out.contains("compile_error !"));
    }

    #[test]
    fn enum_errors() {
        let out = expand(quote! {
            enum Input {
                A,
                B,
            }
        })
        .to_string();

        assert!(out.contains("compile_error !"));
    }

    #[test]
    fn tuple_struct_errors() {
        let out = expand(quote! {
            struct Input(u32, u32);
        })
        .to_string();

        assert!(out.contains("compile_error !"));
    }

    #[test]
    fn missing_parse_attr_errors() {
        let out = expand(quote! {
            struct Input {
                number: u32,
            }
        })
        .to_string();

        assert!(out.contains("compile_error !"));
    }
}
