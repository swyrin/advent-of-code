use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Error, ItemFn, ReturnType, Type};

use crate::aoc::sample::{Sample, extract_samples, mark_for_expansion};

pub fn expand(attribute: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    if !attribute.is_empty() {
        return Err(Error::new(Span::call_site(), "#[part] does not accept arguments"));
    }

    let mut function: ItemFn = syn::parse2(item)?;
    mark_for_expansion(&mut function.attrs);
    let samples = extract_samples(&function.attrs)?;
    let input_type = input_type(&function)?;

    Ok(generate(function, input_type, samples))
}

fn input_type(function: &ItemFn) -> syn::Result<Type> {
    let signature = &function.sig;

    if signature.constness.is_some() {
        return Err(Error::new_spanned(signature, "#[part] functions cannot be const"));
    }

    if signature.asyncness.is_some() {
        return Err(Error::new_spanned(signature, "#[part] functions cannot be async"));
    }

    if matches!(signature.safety, syn::Safety::Unsafe(_)) {
        return Err(Error::new_spanned(signature, "#[part] functions cannot be unsafe"));
    }

    if signature.abi.is_some() {
        return Err(Error::new_spanned(signature, "#[part] functions cannot use an ABI"));
    }

    if signature.variadic.is_some() {
        return Err(Error::new_spanned(signature, "#[part] functions cannot be variadic"));
    }

    if !signature.generics.params.is_empty() {
        return Err(Error::new_spanned(signature, "#[part] functions cannot be generic"));
    }

    if matches!(signature.output, ReturnType::Default)
        || matches!(
            &signature.output,
            ReturnType::Type(_, output) if matches!(output.as_ref(), Type::Tuple(tuple) if tuple.elems.is_empty())
        )
    {
        return Err(Error::new_spanned(&signature.output, "#[part] functions must return Display"));
    }

    let Some(syn::FnArg::Typed(argument)) = signature.inputs.first() else {
        return Err(Error::new_spanned(
            &signature.inputs,
            "#[part] functions must accept exactly one shared reference",
        ));
    };

    if signature.inputs.len() != 1 {
        return Err(Error::new_spanned(
            &signature.inputs,
            "#[part] functions must accept exactly one shared reference",
        ));
    }

    let Type::Reference(reference) = argument.ty.as_ref() else {
        return Err(Error::new_spanned(&argument.ty, "#[part] input must be a shared reference"));
    };

    if reference.mutability.is_some() {
        return Err(Error::new_spanned(&argument.ty, "#[part] input must be a shared reference"));
    }

    Ok(*reference.elem.clone())
}

fn generate(function: ItemFn, input_type: Type, samples: Vec<Sample>) -> TokenStream {
    let function_name = &function.sig.ident;
    let function_name_string = function_name.to_string();
    let support_name = Ident::new(&format!("__aoc_{function_name}"), Span::call_site());

    let tests = samples
        .into_iter()
        .enumerate()
        .map(|(index, sample)| {
            let sample_input = sample.input;
            let expected = sample.expected;
            let test_name = Ident::new(&format!("sample_{index}"), Span::call_site());

            quote! {
                #[test]
                fn #test_name() {
                    let input: #input_type = (#sample_input)
                        .parse()
                        .unwrap_or_else(|_| panic!("Failed to parse sample input"));

                    let actual = #function_name(&input);

                    assert_eq!(
                        actual.to_string(),
                        (#expected).to_string(),
                        "Sample failed for {}",
                        #function_name_string,
                    );
                }
            }
        })
        .collect::<Vec<_>>();

    let test_module = if tests.is_empty() {
        quote! {
            #[cfg(test)]
            mod tests {}
        }
    } else {
        quote! {
            #[cfg(test)]
            mod tests {
                use super::*;

                #(#tests)*
            }
        }
    };

    quote! {
        #[forbid(unsafe_code)]
        #function

        #[doc(hidden)]
        mod #support_name {
            use super::*;

            fn run(content: &str) {
                let input: #input_type = content.parse().unwrap_or_else(|_| {
                    panic!("Failed to parse input.txt for {}", #function_name_string)
                });

                let result = super::#function_name(&input);

                println!("Result of {}: {}", #function_name_string, result);
            }

            ::macros::inventory::submit! {
                ::macros::AocPart {
                    name: #function_name_string,
                    run,
                }
            }

            #test_module
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn generates_registration_and_sample_tests() {
        let output = expand(quote! {}, quote! {
            #[sample(input = "1", expected = "1")]
            #[sample(input = "2", expected = "2")]
            fn part_one(Input { value }: &Input) -> impl std::fmt::Display {
                value
            }
        })
        .unwrap()
        .to_string();

        assert!(output.contains("inventory :: submit"));
        assert!(output.contains("sample_0"));
        assert!(output.contains("sample_1"));
        assert!(output.contains("__aoc_part_sample"));
    }

    #[test]
    fn rejects_extra_parameter() {
        let error = expand(quote! {}, quote! {
            fn part_one(input: &Input, extra: usize) -> impl std::fmt::Display {
                input.value
            }
        })
        .unwrap_err();

        assert!(error.to_string().contains("exactly one shared reference"));
    }

    #[test]
    fn rejects_mutable_input() {
        let error = expand(quote! {}, quote! {
            fn part_one(input: &mut Input) -> impl std::fmt::Display {
                input.value
            }
        })
        .unwrap_err();

        assert!(error.to_string().contains("shared reference"));
    }

    #[test]
    fn rejects_attribute_arguments() {
        let error = expand(quote! { 1 }, quote! {
            fn part_one(input: &Input) -> impl std::fmt::Display {
                input.value
            }
        })
        .unwrap_err();

        assert!(error.to_string().contains("does not accept arguments"));
    }
}
