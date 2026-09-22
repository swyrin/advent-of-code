use std::collections::HashSet;

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
        let content = attr.parse_args::<TokenStream>()?;

        let mut entries = split_entries(&content);

        if entries.len() > 1 && entries.last().is_some_and(TokenStream::is_empty) {
            entries.pop();
        }

        if entries.is_empty() || entries.iter().any(TokenStream::is_empty) {
            return Err(Error::new_spanned(attr, "`#[parse(...)]` cannot be empty"));
        }

        let mut definitions = Vec::new();

        for entry in &entries[..entries.len() - 1] {
            definitions.push(parse_named(entry)?);
        }

        let final_expr = entries.pop().unwrap();

        validate_final(&final_expr)?;

        Ok(Self {
            definitions,
            final_expr,
        })
    }

    pub fn uses_sections(&self) -> bool {
        self.expr_uses_sections(&self.final_expr, &mut HashSet::new())
    }

    fn expr_uses_sections(&self, expr: &TokenStream, seen: &mut HashSet<String>) -> bool {
        for token in expr.clone() {
            match token {
                TokenTree::Group(group) => {
                    if self.expr_uses_sections(&group.stream(), seen) {
                        return true;
                    }
                },

                TokenTree::Ident(ident) => {
                    if ident == "section" || ident == "sections" {
                        return true;
                    }

                    if seen.insert(ident.to_string())
                        && let Some(definition) =
                            self.definitions.iter().find(|def| def.name == ident)
                        && self.expr_uses_sections(&definition.expr, seen)
                    {
                        return true;
                    }
                },

                _ => {},
            }
        }

        false
    }
}

fn split_entries(content: &TokenStream) -> Vec<TokenStream> {
    let mut entries = vec![TokenStream::new()];

    for token in content.clone() {
        if let TokenTree::Punct(punct) = &token
            && punct.as_char() == ','
        {
            entries.push(TokenStream::new());
            continue;
        }

        entries.last_mut().expect("there is always at least one entry").extend([token]);
    }

    entries
}

fn parse_named(entry: &TokenStream) -> Result<ParseDefinition> {
    let mut tokens = entry.clone().into_iter();

    let Some(TokenTree::Ident(name)) = tokens.next() else {
        return Err(Error::new_spanned(entry, "only the final parser may be unnamed"));
    };

    let Some(TokenTree::Punct(punct)) = tokens.next() else {
        return Err(Error::new_spanned(entry, "expected `key = parser`"));
    };

    if punct.as_char() != '=' {
        return Err(Error::new_spanned(entry, "expected `key = parser`"));
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

    let Some(TokenTree::Ident(name)) = tokens.next() else {
        return Ok(());
    };

    let Some(TokenTree::Punct(first)) = tokens.next() else {
        return Ok(());
    };

    if first.as_char() != '=' {
        return Ok(());
    }

    if let Some(TokenTree::Punct(second)) = tokens.next()
        && second.as_char() == '>'
    {
        return Ok(());
    }

    Err(Error::new_spanned(
        final_expr,
        format!(
            "the parser assigned to the field must be the last one and unnamed; \
             `{name}` should not have a name"
        ),
    ))
}
