use proc_macro::TokenStream;
use quote::quote;
use syn::visit::Visit;
use syn::DeriveInput;
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

        let block = if let Some(block) = func.default.clone() {
            quote! { #block }
        } else {
            let mut block = quote! {{ unsafe { self.0.#name.map(|f| f(self.0.get_this(), #args)) }}};
            if let Some(syn::Type::Path(TypePath { path, .. })) = ty {
                if path.is_ident("bool") {
                    block = quote! {{ unsafe { self.0.#name.map(|f| f(self.0.get_this(), #args) == 1) }}};
                }
            }
            block
        };

        let attrs = quote! { #(#attrs)* };
        quote! {
            #attrs
            pub #sig #block
        }
    });

    let result = quote! {
        #(#expanded)*
    };

    result.into()
}

#[proc_macro_derive(FfiRc)]
/// Filters out the types that are not reference counted.
pub fn derive_ffi_rc(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let mut visitor = TypeVisitor::default();
    visitor.visit_derive_input(&input);

    if visitor.has_rc {
        quote! {
            impl crate::FfiRc for #name {}
        }
    } else {
        quote! {}
    }
    .into()
}

#[derive(Default)]
struct TypeVisitor {
    has_rc: bool,
}

impl<'ast> Visit<'ast> for TypeVisitor {
    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        if i.path.segments.last().map_or(false, |s| {
            s.ident == "RefCounted"
                || s.ident == "cef_base_ref_counted_t"
                || s.ident == "_cef_base_ref_counted_t"
                || s.ident == "cef_base_scoped_t"
                || s.ident == "cef_view_delegate_t"
                || s.ident == "cef_view_t"
                || s.ident == "cef_label_button_t"
                || s.ident == "cef_layout_t"
                || s.ident == "cef_button_t"
                || s.ident == "cef_button_delegate_t"
                || s.ident == "cef_panel_delegate_t"
                || s.ident == "cef_panel_t"
                || s.ident == "cef_translator_test_ref_ptr_library_t"
                || s.ident == "cef_translator_test_ref_ptr_library_child_t"
                || s.ident == "cef_translator_test_ref_ptr_client_t"
                || s.ident == "cef_translator_test_scoped_library_t"
                || s.ident == "cef_translator_test_scoped_library_child_t"
                || s.ident == "cef_translator_test_scoped_client_t"
                || s.ident == "cef_preference_manager_t"
        }) {
            self.has_rc = true;
        }
        syn::visit::visit_type_path(self, i);
    }
}