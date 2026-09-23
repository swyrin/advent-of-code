use proc_macro::TokenStream;
use proc_macro2::{Delimiter, TokenTree};
use quote::quote;
use syn::{Error, ItemFn, Result};

use crate::aoc::parser::AocInput;
use crate::aoc::part::{Part, extract_part, generate_part, input_type_from_function};
use crate::aoc::sample::extract_sample;

pub fn expand(input: TokenStream) -> TokenStream {
    let raw: proc_macro2::TokenStream = input.clone().into();

    match syn::parse2::<AocInput>(raw.clone()) {
        Ok(parsed) => match expand_impl(parsed) {
            Ok(tokens) => tokens.into(),
            Err(error) => fallback_verbatim(raw, error).into(),
        },

        Err(parse_error) => recover(raw, parse_error).into(),
    }
}

fn expand_impl(mut input: AocInput) -> Result<proc_macro2::TokenStream> {
    let part_one = extract_part(&mut input.functions, "part_one")?;

    let part_two = extract_part(&mut input.functions, "part_two")?;

    let input_type = match (&part_one, &part_two) {
        (Some(part), _) => input_type_from_function(&part.function)?,

        (_, Some(part)) => input_type_from_function(&part.function)?,

        (None, None) => {
            return Err(Error::new(
                proc_macro2::Span::call_site(),
                "expected `part_one` or `part_two`",
            ));
        },
    };

    let (part_one_code, part_one_run) = generate_part(part_one, &input_type, 1);

    let (part_two_code, part_two_run) = generate_part(part_two, &input_type, 2);

    let functions = &input.functions;

    Ok(quote! {
        #(#functions)*
        #part_one_code
        #part_two_code

        fn main() {
            let input_content =
                std::fs::read_to_string("input.txt")
                    .expect("No input.txt file");

            let input: #input_type =
                input_content
                    .parse()
                    .expect("Failed to parse input.txt");

            #part_one_run
            #part_two_run
        }
    })
}

fn fallback_verbatim(raw: proc_macro2::TokenStream, error: Error) -> proc_macro2::TokenStream {
    let compile_error = error.into_compile_error();
    let raw = strip_sample_attrs(raw);

    quote! {
        #raw

        fn main() {}

        #compile_error
    }
}

fn strip_sample_attrs(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut out: Vec<TokenTree> = Vec::new();
    let mut tokens = input.into_iter().peekable();

    while let Some(token) = tokens.next() {
        let is_hash = matches!(&token, TokenTree::Punct(punct) if punct.as_char() == '#');

        if is_hash
            && let Some(TokenTree::Group(group)) = tokens.peek()
            && group.delimiter() == Delimiter::Bracket
            && group
                .stream()
                .into_iter()
                .next()
                .is_some_and(|first| matches!(first, TokenTree::Ident(ident) if ident == "sample"))
        {
            // Drop the `#` and the `[sample(...)]` group.
            tokens.next();
            continue;
        }

        out.push(token);
    }

    out.into_iter().collect()
}

fn split_function_chunks(input: proc_macro2::TokenStream) -> Vec<proc_macro2::TokenStream> {
    let mut chunks = Vec::new();
    let mut current: Vec<TokenTree> = Vec::new();
    let mut seen_fn = false;

    for token in input {
        match &token {
            TokenTree::Ident(ident) if ident == "fn" => {
                if seen_fn && !current.is_empty() {
                    // Previous `fn` never got a body (incomplete input);
                    // flush it as its own broken chunk.
                    let chunk: proc_macro2::TokenStream = current.drain(..).collect();
                    chunks.push(chunk);
                }

                seen_fn = true;
                current.push(token);
            },

            TokenTree::Group(group) if seen_fn && group.delimiter() == Delimiter::Brace => {
                current.push(token);

                let chunk: proc_macro2::TokenStream = current.drain(..).collect();
                chunks.push(chunk);
                seen_fn = false;
            },

            _ => {
                current.push(token);
            },
        }
    }

    if !current.is_empty() {
        let chunk: proc_macro2::TokenStream = current.into_iter().collect();
        chunks.push(chunk);
    }

    chunks
}

fn extract_part_tolerant(
    functions: &mut Vec<ItemFn>,
    broken: &mut Vec<proc_macro2::TokenStream>,
    errors: &mut Vec<Error>,
    name: &str,
) -> Option<Part> {
    let index = functions.iter().position(|function| function.sig.ident == name)?;

    let mut function = functions.remove(index);

    match extract_sample(&mut function.attrs) {
        Ok(sample) => Some(Part {
            function,
            sample,
        }),

        Err(error) => {
            errors.push(error);
            broken.push(quote! { #function });
            None
        },
    }
}

// noinspection ALL
fn recover(raw: proc_macro2::TokenStream, parse_error: Error) -> proc_macro2::TokenStream {
    let mut errors = vec![parse_error];
    let mut broken: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut good: Vec<ItemFn> = Vec::new();

    let chunks = split_function_chunks(raw.clone());

    if chunks.is_empty() {
        let compile_error =
            errors.into_iter().map(|error| error.into_compile_error()).collect::<Vec<_>>();

        return quote! {
            fn main() {}

            #(#compile_error)*
        };
    }

    for chunk in chunks {
        match syn::parse2::<ItemFn>(chunk.clone()) {
            Ok(function) => good.push(function),

            Err(error) => {
                errors.push(error);
                broken.push(strip_sample_attrs(chunk));
            },
        }
    }

    let mut functions = good;

    let mut part_one = extract_part_tolerant(&mut functions, &mut broken, &mut errors, "part_one");
    let mut part_two = extract_part_tolerant(&mut functions, &mut broken, &mut errors, "part_two");

    // Infer the input type from whichever good part survived. If its
    // signature is itself broken, demote that part to `broken` and try the
    // other one.
    let input_type = match (&part_one, &part_two) {
        (Some(part), _) => match input_type_from_function(&part.function) {
            Ok(ty) => Some(ty),

            Err(error) => {
                errors.push(error);

                if let Some(part) = part_one.take() {
                    let function = part.function;
                    broken.push(quote! { #function });
                }

                match &part_two {
                    Some(part) => match input_type_from_function(&part.function) {
                        Ok(ty) => Some(ty),

                        Err(error) => {
                            errors.push(error);

                            if let Some(part) = part_two.take() {
                                let function = part.function;
                                broken.push(quote! { #function });
                            }

                            None
                        },
                    },

                    None => None,
                }
            },
        },

        (_, Some(part)) => match input_type_from_function(&part.function) {
            Ok(ty) => Some(ty),

            Err(error) => {
                errors.push(error);

                if let Some(part) = part_two.take() {
                    let function = part.function;
                    broken.push(quote! { #function });
                }

                None
            },
        },

        (None, None) => None,
    };

    let compile_errors =
        errors.into_iter().map(|error| error.into_compile_error()).collect::<Vec<_>>();

    match input_type {
        Some(input_type) => {
            let (part_one_code, part_one_run) = generate_part(part_one, &input_type, 1);
            let (part_two_code, part_two_run) = generate_part(part_two, &input_type, 2);

            quote! {
                #(#functions)*
                #part_one_code
                #part_two_code
                #(#broken)*

                fn main() {
                    let input_content =
                        std::fs::read_to_string("input.txt")
                            .expect("No input.txt file");

                    let input: #input_type =
                        input_content
                            .parse()
                            .expect("Failed to parse input.txt");

                    #part_one_run
                    #part_two_run
                }

                #(#compile_errors)*
            }
        },

        None => {
            // We couldn't figure out the input type (e.g. both parts are
            // broken), so emit everything we have plus a stub `main` to keep
            // the crate well-formed for the IDE.
            let mut all_good: Vec<proc_macro2::TokenStream> =
                functions.into_iter().map(|function| quote! { #function }).collect();

            if let Some(part) = part_one {
                let function = part.function;
                all_good.push(quote! { #function });
            }

            if let Some(part) = part_two {
                let function = part.function;
                all_good.push(quote! { #function });
            }

            quote! {
                #(#all_good)*
                #(#broken)*

                fn main() {}

                #(#compile_errors)*
            }
        },
    }
}
