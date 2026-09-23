use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Result, parse_macro_input, parse_quote};

use super::parser::ParseSpec;

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

struct FieldSpec {
    name: syn::Ident,
    ty: syn::Type,
}

fn expand_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
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

    if fields.is_empty() {
        return Err(Error::new_spanned(struct_name, "AocInput requires at least one field"));
    }

    let mut specs = Vec::new();

    for field in &fields {
        let name = field.ident.as_ref().unwrap().clone();
        let ty = field.ty.clone();

        let parse_attr = field
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("parse"))
            .ok_or_else(|| Error::new_spanned(field, "missing `#[parse(...)]` attribute"))?;

        if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("trust_me")) {
            return Err(Error::new_spanned(
                attr,
                "`#[trust_me]` has been removed; wrap the parser with `section(...)` instead",
            ));
        }

        specs.push((
            FieldSpec {
                name,
                ty,
            },
            ParseSpec::parse(parse_attr)?,
        ));
    }

    if specs.len() > 1 {
        for (index, (_, spec)) in specs.iter().enumerate() {
            if !spec.uses_sections() {
                return Err(Error::new_spanned(
                    &fields[index],
                    "in a multi-field `AocInput` struct, \
                     each field's parser must use `section` or `sections`",
                ));
            }
        }
    }

    let field_names = specs.iter().map(|(field, _)| field.name.clone()).collect::<Vec<_>>();
    let field_tys = specs.iter().map(|(field, _)| field.ty.clone()).collect::<Vec<_>>();
    let field_strings = field_names.iter().map(|name| name.to_string()).collect::<Vec<_>>();

    let debug_impl = {
        let mut generics = input.generics.clone();
        let where_clause = generics.make_where_clause();

        for ty in &field_tys {
            where_clause.predicates.push(parse_quote!(#ty: ::std::fmt::Debug));
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        quote! {
            impl #impl_generics ::std::fmt::Debug for #struct_name #ty_generics #where_clause {
                fn fmt(
                    &self,
                    f: &mut ::std::fmt::Formatter<'_>,
                ) -> ::std::fmt::Result {
                    f.debug_struct(::std::stringify!(#struct_name))
                        #(.field(#field_strings, &self.#field_names))*
                        .finish()
                }
            }
        }
    };

    let clone_impl = {
        let mut generics = input.generics.clone();
        let where_clause = generics.make_where_clause();

        for ty in &field_tys {
            where_clause.predicates.push(parse_quote!(#ty: ::std::clone::Clone));
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        quote! {
            impl #impl_generics ::std::clone::Clone for #struct_name #ty_generics #where_clause {
                fn clone(&self) -> Self {
                    Self {
                        #(#field_names: ::std::clone::Clone::clone(&self.#field_names)),*
                    }
                }
            }
        }
    };

    let bindings = specs.iter().map(|(field, _)| field.name.clone()).collect::<Vec<_>>();

    let constructor = quote!(Ok(Self {
        #(#bindings),*
    }));

    let from_str_body = if specs.len() == 1 {
        let (field, spec) = &specs[0];
        let name = &field.name;

        let definitions = spec.definitions.iter().map(|definition| {
            let name = &definition.name;
            let expr = &definition.expr;

            quote! {
                let #name = macros::aoc_parse::parser!(#expr);
            }
        });

        let final_expr = &spec.final_expr;

        quote! {
            #(#definitions)*

            let __aoc_final =
                macros::aoc_parse::parser!(#final_expr);

            let #name =
                __aoc_final.parse(s)?;

            Ok(Self {
                #name,
            })
        }
    } else {
        let parses = specs.iter().enumerate().map(|(index, (_, spec))| {
            let binding = &bindings[index];

            let definitions = spec.definitions.iter().map(|definition| {
                let name = &definition.name;
                let expr = &definition.expr;

                quote! {
                    let #name = macros::aoc_parse::parser!(#expr);
                }
            });

            let final_expr = &spec.final_expr;

            if index + 1 < specs.len() {
                quote! {
                    let #binding = {
                        #(#definitions)*

                        let __aoc_parser =
                            macros::aoc_parse::parser!(#final_expr);

                        let (__aoc_take, __aoc_value) = macros::parse_sections(
                            &__aoc_parser,
                            __aoc_sections.get(__aoc_pos..).unwrap_or(&[]),
                        )?;

                        __aoc_pos += __aoc_take;
                        __aoc_value
                    };
                }
            } else {
                quote! {
                    let #binding = {
                        #(#definitions)*

                        let __aoc_parser =
                            macros::aoc_parse::parser!(#final_expr);

                        __aoc_parser.parse(
                            &__aoc_sections.get(__aoc_pos..).unwrap_or(&[]).join("\n\n"),
                        )?
                    };
                }
            }
        });

        quote! {
            let __aoc_sections = macros::split_sections(s);
            let mut __aoc_pos: usize = 0;

            #(#parses)*

            #constructor
        }
    };

    Ok(quote! {
        #debug_impl
        #clone_impl

        impl std::str::FromStr for #struct_name {
            type Err = macros::aoc_parse::ParseError;

            fn from_str(
                s: &str,
            ) -> Result<Self, Self::Err> {
                use macros::aoc_parse::Parser;
                use macros::aoc_parse::prelude::*;

                #from_str_body
            }
        }
    })
}
