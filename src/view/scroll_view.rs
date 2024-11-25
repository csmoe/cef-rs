use crate::{rc::RefGuard, wrapper, ViewDelegate};
use cef_sys::cef_scroll_view_t;
use cef_wrapper_macro::wrapper_methods;

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

impl ScrollView {
    wrapper_methods!(
        /// See [cef_scroll_view_t::set_content_view]
        fn set_content_view(&mut self, view: crate::View);
        /// See [cef_scroll_view_t::get_content_view]
        fn get_content_view(&self) -> crate::View {
            self.0.get_content_view.map(|f| unsafe {
                let view = f(self.0.get_this());
                crate::View(RefGuard::from_raw(view))
            })
        }
        /// See [cef_scroll_view_t::get_visible_content_rect]
        fn get_visible_content_rect(&self) -> crate::Rect;
        /// See [cef_scroll_view_t::has_horizontal_scrollbar]
        fn has_horizontal_scrollbar(&self) -> bool;
        /// See [cef_scroll_view_t::get_horizontal_scrollbar_height]
        fn get_horizontal_scrollbar_height(&self) -> i32;
        /// See [cef_scroll_view_t::has_vertical_scrollbar]
        fn has_vertical_scrollbar(&self) -> bool;
        /// See [cef_scroll_view_t::get_vertical_scrollbar_width]
        fn get_vertical_scrollbar_width(&self) -> i32;
    );
}
