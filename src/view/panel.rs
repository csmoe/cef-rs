use crate::{add_view_delegate_methods, view::View, wrapper, ViewDelegate};
use cef_sys::cef_panel_create;
use cef_sys::{cef_panel_delegate_t, cef_panel_t};
use cef_wrapper_macro::wrapper_methods;

wrapper!(
    /// See [cef_panel_t] for more documentation.
    #[derive(Debug, Clone)]
    pub struct Panel(cef_panel_t);
);

crate::convert_view! {
    (Panel, as_window, Window)
}

impl Panel {
    /// See [cef_panel_create]
    pub fn create(delegate: impl PanelDelegate) -> crate::Result<Self> {
        unsafe {
            let view = cef_panel_create(PanelDelegate::into_raw(delegate));
            if view.is_null() {
                return Err(crate::Error::NullPtr);
            }
            Ok(Self::from_raw(view))
        }
    }
}

impl Panel {
    wrapper_methods!(
        /// See [cef_panel_t::set_to_fill_layout]
        fn set_to_fill_layout(&mut self) -> crate::FillLayout {
            self.0.set_to_fill_layout.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() {
                    None
                } else {
                    Some(crate::FillLayout::from_raw(v))
                }
            })
        }

        /// See [cef_panel_t::set_to_box_layout]
        fn set_to_box_layout(&mut self, settings: crate::BoxLayoutSettings) -> crate::BoxLayout {
            self.0.set_to_box_layout.and_then(|f| unsafe {
                let v = f(self.0.get_this(), std::ptr::from_ref(&settings.into_raw()));
                if v.is_null() {
                    None
                } else {
                    Some(crate::BoxLayout::from_raw(v))
                }
            })
        }

        /// See [cef_panel_t::get_layout]
        fn get_layout(&self) -> crate::Layout {
            self.0.get_layout.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() {
                    None
                } else {
                    Some(crate::Layout::from_raw(v))
                }
            })
        }

        /// See [cef_panel_t::layout]
        fn layout(&mut self);

        /// See [cef_panel_t::add_child_view]
        fn add_child_view(&mut self, view: crate::View);

        /// See [cef_panel_t::add_child_view_at]
        fn add_child_view_at(&mut self, view: crate::View, index: i32);

        /// See [cef_panel_t::reorder_child_view]
        fn reorder_child_view(&mut self, view: crate::View, index: i32);

        /// See [cef_panel_t::remove_child_view]
        fn remove_child_view(&mut self, view: crate::View);

        /// See [cef_panel_t::remove_all_child_views]
        fn remove_all_child_views(&mut self);

        /// See [cef_panel_t::get_child_view_count]
        fn get_child_view_count(&self) -> usize;

        /// See [cef_panel_t::get_child_view_at]
        fn get_child_view_at(&self, index: i32) -> crate::View {
            self.0.get_child_view_at.and_then(|f| unsafe {
                let v = f(self.0.get_this(), index);
                if v.is_null() {
                    None
                } else {
                    Some(View::from_raw(v))
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
