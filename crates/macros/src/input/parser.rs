use proc_macro2::{TokenStream, TokenTree};
use syn::{Error, Ident, Result};

pub struct ParseDefinition {
    pub name: Ident,
    pub expr: TokenStream,
}

pub struct ParseSpec {
    pub definitions: Vec<ParseDefinition>,
    pub final_expr: TokenStream,
}

impl ParseSpec {
    pub fn parse(attr: &syn::Attribute) -> Result<Self> {
        let entries = attr.parse_args_with(
            syn::punctuated::Punctuated::<TokenStream, syn::Token![,]>::parse_terminated,
        )?;

        if entries.is_empty() {
            return Err(Error::new_spanned(attr, "`#[parse(...)]` cannot be empty"));
        }

        let mut definitions = Vec::new();

        // Everything except the last entry must be `name = parser`.
        for entry in entries.iter().take(entries.len() - 1) {
            definitions.push(parse_named(entry)?);
        }

        // The last entry must be the unnamed parser.
        let final_expr = entries.last().unwrap().clone();

        validate_final(&final_expr)?;

        Ok(Self {
            definitions,
            final_expr,
        })
    }
}

fn parse_named(entry: &TokenStream) -> Result<ParseDefinition> {
    let mut tokens = entry.clone().into_iter();

    let Some(TokenTree::Ident(name)) = tokens.next() else {
        return Err(Error::new_spanned(entry, "only the final parser may be unnamed"));
    };

    let Some(TokenTree::Punct(punct)) = tokens.next() else {
        return Err(Error::new_spanned(entry, "expected `name = parser`"));
    };

    if punct.as_char() != '=' {
        return Err(Error::new_spanned(entry, "expected `name = parser`"));
    }

    let expr: TokenStream = tokens.collect();

    if expr.is_empty() {
        return Err(Error::new_spanned(entry, "missing parser expression"));
    }

    Ok(ParseDefinition {
        name,
        expr,
    })
}

fn validate_final(final_expr: &TokenStream) -> Result<()> {
    let mut tokens = final_expr.clone().into_iter();

    // If the final expression starts with:
    //
    //     foo = ...
    //
    // then it was accidentally named.
    let Some(TokenTree::Ident(name)) = tokens.next() else {
        return Ok(());
    };

    let Some(TokenTree::Punct(punct)) = tokens.next() else {
        return Ok(());
    };

    if punct.as_char() == '=' {
        return Err(Error::new_spanned(
            final_expr,
            format!(
                "the parser assigned to the field must be unnamed; \
                 `{name}` should not have a name"
            ),
        ));
    }

    Ok(())
}
