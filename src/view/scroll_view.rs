use crate::prelude::*;
use crate::ViewDelegate;
use cef_wrapper_macro::wrapper;

#[wrapper]
/// See [cef_scroll_view_t] for more documentation.
#[derive(Debug, Clone)]
pub struct CefScrollView(cef_scroll_view_t);

impl CefScrollView {
    /// See [cef_sys::cef_scroll_view_create]
    pub fn create(delegate: impl ViewDelegate) -> Result<Self> {
        unsafe {
            let view = cef_sys::cef_scroll_view_create(delegate.into_raw());
            if view.is_null() {
                return Err(Error::NullPtr);
            }
            Ok(Self::from(view))
        }
    }
}

impl CefScrollView {
    wrapper_methods!(
        /// See [cef_scroll_view_t::set_content_view]
        fn set_content_view(&mut self, view: crate::CefView);
        /// See [cef_scroll_view_t::get_content_view]
        fn get_content_view(&self) -> crate::CefView {
            get_content_view.map(|f| unsafe {
                let view = f(self.get_this());
                crate::CefView::from(view)
            })
        }
        /// See [cef_scroll_view_t::get_visible_content_rect]
        fn get_visible_content_rect(&self) -> crate::CefRect;
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
