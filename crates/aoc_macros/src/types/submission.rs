use syn::parse::Parse;

/// The submission type.
pub struct SubmissionArgs {
    /// String verbatim of the input.
    /// Usually double-clicking on the sample input box should work.
    pub sample_in: syn::LitStr,

    /// The highlighted number, near the end of the statement.
    pub sample_out: syn::LitInt,
}

impl Parse for SubmissionArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut sample_in = None;
        let mut sample_out = None;

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
                    sample_out = Some(input.parse::<syn::LitInt>()?);
                }
                other => {
                    return Err(input.error(format!(
                        "Unknown argument `{}` (expected `sample_in`, `sample_out`)",
                        other
                    )));
                }
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self {
            sample_in: sample_in.ok_or_else(|| input.error("Missing `sample_in`"))?,
            sample_out: sample_out.ok_or_else(|| input.error("Missing `sample_out`"))?,
        })
    }
}
