use syn::parse::Parse;

/// The submission type.
pub enum SubmissionArgs {
    Sample {
        /// String verbatim of the input.
        /// Usually double-clicking on the sample input box should work.
        sample_in: syn::LitStr,

        /// The expected sample answer, as any expression implementing `ToString`.
        sample_out: syn::Expr,
    },
    Ignored {
        /// Why this part cannot be tested automatically.
        reason: syn::LitStr,
    },
}

impl Parse for SubmissionArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut sample_in = None;
        let mut sample_out = None;
        let mut ignore = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match ident.to_string().as_str() {
                "sample_in" => {
                    if sample_in.is_some() {
                        return Err(input.error("`sample_in` specified more than once"));
                    }
                    sample_in = Some(input.parse::<syn::LitStr>()?);
                }
                "sample_out" => {
                    if sample_out.is_some() {
                        return Err(input.error("`sample_out` specified more than once"));
                    }
                    sample_out = Some(input.parse::<syn::Expr>()?);
                }
                "ignore" => {
                    if ignore.is_some() {
                        return Err(input.error("`ignore` specified more than once"));
                    }
                    ignore = Some(input.parse::<syn::LitStr>()?);
                }
                other => {
                    return Err(input.error(format!(
                        "Unknown argument `{}` (expected `sample_in`, `sample_out`, `ignore`)",
                        other
                    )));
                }
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        match (sample_in, sample_out, ignore) {
            (Some(sample_in), Some(sample_out), None) => Ok(Self::Sample {
                sample_in,
                sample_out,
            }),
            (None, None, Some(reason)) => Ok(Self::Ignored { reason }),
            (_, _, Some(_)) => {
                Err(input.error("`ignore` cannot be combined with `sample_in` or `sample_out`"))
            }
            (Some(_), None, None) => Err(input.error("Missing `sample_out`")),
            (None, Some(_), None) => Err(input.error("Missing `sample_in`")),
            (None, None, None) => {
                Err(input.error("Expected either `sample_in` and `sample_out`, or `ignore`"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SubmissionArgs;

    #[test]
    fn accepts_sample_input_and_output() {
        let args = syn::parse_str::<SubmissionArgs>(r#"sample_in = "42", sample_out = 2"#);

        assert!(matches!(args, Ok(SubmissionArgs::Sample { .. })));
    }

    #[test]
    fn accepts_an_ignore_reason() {
        let args = syn::parse_str::<SubmissionArgs>(r#"ignore = "Requires manual inspection""#);

        assert!(matches!(args, Ok(SubmissionArgs::Ignored { .. })));
    }

    #[test]
    fn rejects_mixed_variants() {
        let error = syn::parse_str::<SubmissionArgs>(
            r#"sample_in = "42", sample_out = 2, ignore = "Manual""#,
        )
        .err()
        .expect("mixed variants should fail");

        assert!(error.to_string().contains("cannot be combined"));
    }
}
