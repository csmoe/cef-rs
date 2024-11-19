use crate::{rc::RefGuard, wrapper, ViewDelegate};
use cef_sys::cef_scroll_view_t;

wrapper! {
    #[doc = "See [cef_scroll_view_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct ScrollView(cef_scroll_view_t);
}

impl ScrollView {
    pub fn create(delegate: impl ViewDelegate) -> Self {
        unsafe {
            let view = cef_sys::cef_scroll_view_create(delegate.into_raw());
            Self(RefGuard::from_raw(view))
        }
    }
}