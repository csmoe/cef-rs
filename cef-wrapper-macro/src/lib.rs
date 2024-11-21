use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Pat, PatType, ReturnType, TraitItemFn, TypePath,
};

struct MultipleFunctions {
    functions: Vec<TraitItemFn>,
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

fn wrap_return_type(return_type: &ReturnType) -> (Option<syn::Type>, proc_macro2::TokenStream) {
    match return_type {
        ReturnType::Default => (None, quote! { -> Option<()> }),
        ReturnType::Type(arrow, ty) => {
            let wrapped = quote! { Option<#ty> };
            (Some(*ty.clone()), quote! { #arrow #wrapped })
        }
    }
}

#[proc_macro]
pub fn wrapper_methods(input: TokenStream) -> TokenStream {
    let MultipleFunctions { functions } = parse_macro_input!(input as MultipleFunctions);

    let expanded = functions.iter().map(|func| {
        let mut sig = func.sig.clone();
        let wrapped_output = wrap_return_type(&sig.output);
        let name = &sig.ident;
        let attrs = &func.attrs;

        let args = (&sig.inputs).iter().filter_map(|arg| {
            if let syn::FnArg::Typed(PatType { pat, .. }) = arg {
                if let Pat::Ident(pat_ident) = &**pat {
                    Some(quote! { #pat_ident.into() })
                } else {
                    None
                }
            } else {
                None
            }
        });
        let (ty, wrapped_output) = wrapped_output;
        sig.output = syn::parse2(wrapped_output).unwrap();

        let args = quote! { #(#args,)* };

        let mut body = quote! { self.0.#name.map(|f|f(self.0.get_raw(), #args)) };
        if let Some(syn::Type::Path(TypePath { path, .. })) = ty {
            if path.is_ident("bool") {
                body = quote! { self.0.#name.map(|f| f(self.0.get_raw(), #args) == 1) };
            }
        }

        let attrs = quote! { #(#attrs)* };
        quote! {
            #attrs
            pub #sig {
                unsafe { #body }
            }
        }
    });

    let result = quote! {
        #(#expanded)*
    };

    result.into()
}
