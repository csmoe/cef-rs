use crate::{add_view_delegate_methods, rc::RefGuard, string::CefString};

use super::ViewDelegate;

crate::wrapper!(
    #[doc = "See `[cef_sys::cef_button_t]` for more documentation."]
    #[derive(Debug, Clone)]
    pub struct Button(cef_sys::cef_button_t);
);

crate::wrapper!(
    #[doc = "See `[cef_sys::cef_label_button_t]` for more documentation."]
    #[derive(Debug, Clone)]
    pub struct LabelButton(cef_sys::cef_label_button_t);
);

pub trait ButtonDelegate: ViewDelegate {
    fn into_raw(self) -> *mut cef_sys::cef_button_delegate_t {
        let mut object: cef_sys::cef_button_delegate_t = unsafe { std::mem::zeroed() };
        let view = &mut object.base;
        add_view_delegate_methods!(view);
        RcImpl::new(object, self).cast()
    }
}

impl LabelButton {
    pub fn create(delegate: impl ButtonDelegate, text: CefString) -> Self {
        unsafe {
            // TODO: ui thread restriction
            let button = cef_sys::cef_label_button_create(
                <_ as ButtonDelegate>::into_raw(delegate),
                std::ptr::from_ref(&text.as_raw()),
            );
            Self(RefGuard::from_raw(button))
        }
    }
}

crate::wrapper!(
    #[doc = "See `[cef_sys::cef_menu_button_t]` for more documentation."]
    #[derive(Debug, Clone)]
    pub struct MenuButton(cef_sys::cef_menu_button_t);
);

crate::convert_view! {
    (Button, as_label_button, LabelButton),
    (LabelButton, as_menu_button, MenuButton)
}
