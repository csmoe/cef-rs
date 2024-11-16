use cef_sys::{cef_panel_delegate_t, cef_panel_t};

use crate::{add_view_delegate_methods, view::View, wrapper, ViewDelegate, Window};

wrapper!(
    #[doc = "See [cef_panel_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct Panel(cef_panel_t);
);

crate::convert_view! {
    (Panel, as_window, Window)
}

impl Panel {
    pub fn add_child_view(&self, view: View) {
        if let Some(f) = self.0.add_child_view {
            unsafe { f(self.0.get_raw(), view.0.into_raw()) }
        }
    }

    pub fn as_window(&self) -> Option<Window> {
        self.0.as_window.and_then(|f| {
            let p = unsafe { f(self.0.get_raw()) };
            if p.is_null() {
                None
            } else {
                Some(unsafe { Window::from_raw(p) })
            }
        })
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
