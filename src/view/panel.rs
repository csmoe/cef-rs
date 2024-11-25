use cef_sys::cef_panel_create;
use cef_sys::{cef_panel_delegate_t, cef_panel_t};

use crate::rc::RefGuard;
use crate::{add_view_delegate_methods, view::View, wrapper, ViewDelegate};

wrapper!(
    /// See [cef_panel_t] for more documentation.
    #[derive(Debug, Clone)]
    pub struct Panel(cef_panel_t);
);

crate::convert_view! {
    (Panel, as_window, Window)
}

impl Panel {
    pub fn create(delegate: impl PanelDelegate) -> crate::Result<Self> {
        unsafe {
            let view = cef_panel_create(PanelDelegate::into_raw(delegate));
            if view.is_null() {
                return Err(crate::Error::NullPtr);
            }
            Ok(Self::from_raw(view))
        }
    }

    pub fn add_child_view(&self, view: View) {
        if let Some(f) = self.0.add_child_view {
            unsafe { f(self.0.get_this(), view.0.into_raw()) }
        }
    }
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
