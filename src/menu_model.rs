use crate::Result;
use crate::{add_view_delegate_methods, rc::RcImpl, string::CefString, wrapper};
use cef_sys::cef_menu_model_create;
use cef_sys::cef_menu_model_delegate_t;
use cef_sys::cef_menu_model_t;
use cef_wrapper_macro::wrapper_methods;

wrapper! {
    /// See [cef_menu_model_t] for more docs.
    #[derive(Debug, Clone)]
    pub struct MenuModel(cef_menu_model_t);
}

impl MenuModel {
    /// See [cef_menu_model_create] for more docs.
    fn create(delegate: impl MenuModelDelegate) -> Result<Self> {
        unsafe {
            let m = cef_menu_model_create(delegate.into_raw());
            if m.is_null() {
                Err(crate::Error::NullPtr)
            } else {
                Ok(Self::from_raw(m))
            }
        }
    }
}

pub trait MenuModelDelegate: Sized {
    fn into_raw(self) -> *mut cef_menu_model_delegate_t {
        todo!()
    }
}
