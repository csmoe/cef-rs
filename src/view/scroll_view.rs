use crate::{rc::RefGuard, wrapper, ViewDelegate};
use cef_sys::cef_scroll_view_t;

wrapper! {
    /// See [cef_scroll_view_t] for more documentation.
    #[derive(Debug, Clone)]
    pub struct ScrollView(cef_scroll_view_t);
}

impl ScrollView {
    pub fn create(delegate: impl ViewDelegate) -> crate::Result<Self> {
        unsafe {
            let view = cef_sys::cef_scroll_view_create(delegate.into_raw());
            if view.is_null() {
                return Err(crate::Error::NullPtr);
            }
            Ok(Self(RefGuard::from_raw(view)))
        }
    }
}
