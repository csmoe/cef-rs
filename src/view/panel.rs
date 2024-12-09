use crate::prelude::*;
use crate::{add_view_delegate_methods, view::CefView, ViewDelegate};

/// See [cef_panel_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefPanel(cef_panel_t);

crate::convert_view! {
    (CefPanel, as_window, CefWindow)
}

impl CefPanel {
    /// See [cef_panel_create]
    pub fn create(delegate: impl PanelDelegate) -> Result<Self> {
        unsafe {
            let view = cef_panel_create(PanelDelegate::into_raw(delegate));
            if view.is_null() {
                return Err(Error::NullPtr);
            }
            Ok(Self::from_raw(view))
        }
    }
}

impl CefPanel {
    wrapper_methods!(
        /// See [cef_panel_t::set_to_fill_layout]
        fn set_to_fill_layout(&mut self) -> crate::CefFillLayout {
            self.0.set_to_fill_layout.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() {
                    None
                } else {
                    Some(crate::CefFillLayout::from_raw(v))
                }
            })
        }

        /// See [cef_panel_t::set_to_box_layout]
        fn set_to_box_layout(
            &mut self,
            settings: crate::CefBoxLayoutSettings,
        ) -> crate::CefBoxLayout {
            self.0.set_to_box_layout.and_then(|f| unsafe {
                let v = f(self.0.get_this(), std::ptr::from_ref(&settings.into_raw()));
                if v.is_null() {
                    None
                } else {
                    Some(crate::CefBoxLayout::from_raw(v))
                }
            })
        }

        /// See [cef_panel_t::get_layout]
        fn get_layout(&self) -> crate::CefLayout {
            self.0.get_layout.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() {
                    None
                } else {
                    Some(crate::CefLayout::from_raw(v))
                }
            })
        }

        /// See [cef_panel_t::layout]
        fn layout(&mut self);

        /// See [cef_panel_t::add_child_view]
        fn add_child_view(&mut self, view: crate::CefView);

        /// See [cef_panel_t::add_child_view_at]
        fn add_child_view_at(&mut self, view: crate::CefView, index: i32);

        /// See [cef_panel_t::reorder_child_view]
        fn reorder_child_view(&mut self, view: crate::CefView, index: i32);

        /// See [cef_panel_t::remove_child_view]
        fn remove_child_view(&mut self, view: crate::CefView);

        /// See [cef_panel_t::remove_all_child_views]
        fn remove_all_child_views(&mut self);

        /// See [cef_panel_t::get_child_view_count]
        fn get_child_view_count(&self) -> usize;

        /// See [cef_panel_t::get_child_view_at]
        fn get_child_view_at(&self, index: i32) -> crate::CefView {
            self.0.get_child_view_at.and_then(|f| unsafe {
                let v = f(self.0.get_this(), index);
                if v.is_null() {
                    None
                } else {
                    Some(CefView::from_raw(v))
                }
            })
        }
    );
}

/// See [cef_panel_delegate_t] for more documentation.
pub trait PanelDelegate: ViewDelegate {
    fn into_raw(self) -> *mut cef_panel_delegate_t {
        let mut object: cef_panel_delegate_t = unsafe { std::mem::zeroed() };

        let view = &mut object.base;
        add_view_delegate_methods!(view);

        RcImpl::new(object, self).cast()
    }
}
