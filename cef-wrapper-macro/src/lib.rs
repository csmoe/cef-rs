use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, ItemFn, ReturnType};

struct MultipleFunctions {
    functions: Vec<ItemFn>,
}

impl Parse for MultipleFunctions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut functions = Vec::new();
        while !input.is_empty() {
            functions.push(input.parse()?);
        }
        Ok(MultipleFunctions { functions })
    }
}

fn wrap_return_type(return_type: &ReturnType) -> proc_macro2::TokenStream {
    match return_type {
        ReturnType::Default => quote! { -> Option<()> },
        ReturnType::Type(arrow, ty) => {
            let wrapped = quote! { Option<#ty> };
            quote! { #arrow #wrapped }
        }
    }
}

#[proc_macro]
pub fn wrapper_methods(input: TokenStream) -> TokenStream {
    let MultipleFunctions { functions } = parse_macro_input!(input as MultipleFunctions);

    let expanded = functions.iter().map(|func| {
        let vis = &func.vis;
        let mut sig = func.sig.clone();
        let wrapped_output = wrap_return_type(&sig.output);
        sig.output = syn::parse2(wrapped_output).unwrap();

        quote! {
            #vis #sig {
                None
            }
        }
    });

    let result = quote! {
        #(#expanded)*
    };

    result.into()
}
