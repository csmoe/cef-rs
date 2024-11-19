use crate::add_view_delegate_methods;
use crate::{rc::RefGuard, wrapper, ViewDelegate};
use cef_sys::cef_key_event_t;
use cef_sys::cef_textfield_delegate_t;
use cef_sys::cef_textfield_t;

wrapper! {
    #[doc = "See [cef_scroll_view_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct TextField(cef_textfield_t);
}

pub trait TextFieldDelegate: ViewDelegate {
    fn on_key_event(&self, textfield: TextField, event: cef_key_event_t) -> bool {
        false
    }
    fn on_after_user_action(&self, textfield: TextField) {}

    fn into_raw(self) -> *mut cef_textfield_delegate_t {
        let mut object: cef_textfield_delegate_t = unsafe { std::mem::zeroed() };
        let view = &mut object.base;
        add_view_delegate_methods!(view);
        RcImpl::new(object, self).cast()
    }
}

impl TextField {
    pub fn create(delegate: impl TextFieldDelegate) -> crate::Result<Self> {
        unsafe {
            let view = cef_sys::cef_textfield_create(<_ as TextFieldDelegate>::into_raw(delegate));
            if view.is_null() {
                return Err(crate::Error::NullPtr);
            }
            Ok(Self(RefGuard::from_raw(view)))
        }
    }
}
