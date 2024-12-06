use proc_macro::TokenStream;
use quote::quote;
use syn::visit::Visit;
use syn::DeriveInput;
use syn::{
    parse::{Parse, ParseStream},
    Pat, PatType, ReturnType, TraitItemFn, TypePath,
};
use syn::{parse_macro_input, ItemStruct};

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

#[proc_macro_attribute]
pub fn wrapper(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let attrs = &input.attrs;
    let name = &input.ident;
    let sys = &input.fields.iter().next().unwrap().ty;

    let expanded = quote! {
        #(#attrs)*
        pub struct #name(pub(crate) crate::rc::RefGuard<#sys>);

        impl crate::rc::Rc for #sys {
            fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
                &self.base.as_base()
            }
        }

        impl crate::rc::Rc for #name {
            fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
                self.0.as_base()
            }
        }

        impl From<*mut #sys> for #name {
            fn from(ptr: *mut #sys) -> Self {
                unsafe { #name(crate::rc::RefGuard::from_raw(ptr)) }
            }
        }

        impl From<#name> for *mut #sys {
            fn from(value: #name) -> Self {
                unsafe { value.into_raw() }
            }
        }

        impl From<#name> for *const #sys {
            fn from(value: #name) -> Self {
                unsafe { value.into_raw() }
            }
        }

        impl #name {
            #[allow(clippy::missing_safety_doc)]
            pub unsafe fn from_raw(ptr: *mut #sys) -> Self {
                Self(crate::rc::RefGuard::from_raw(ptr))
            }

            #[allow(clippy::missing_safety_doc)]
            pub unsafe fn into_raw(self) -> *mut #sys {
                self.0.into_raw()
            }
        }
    };

    TokenStream::from(expanded)
}
