use std::ffi::c_int;

use cef_sys::{cef_view_delegate_t, cef_view_t};

use crate::{
    rc::{RcImpl, RefGuard},
    wrapper,
};

mod panel;
pub use panel::*;
mod button;
pub use button::*;
mod browser;
pub use browser::*;

wrapper!(
    #[doc = "See [cef_view_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct View(cef_view_t);
);

macro_rules! impl_from_view {
    ($( ($as_field:ident, $target_type:ident) ),*) => {
        $(
            impl From<$crate::view::View> for Option<$crate::$target_type> {
                fn from(value: $crate::view::View) -> Self {
                    value.0.$as_field.and_then(|f| {
                        let v = unsafe { f(value.0.get_raw()) };
                        if v.is_null() {
                            None
                        } else {
                            Some($crate::$target_type(unsafe { $crate::rc::RefGuard::from_raw(v) }))
                        }
                    })
                }
            }
        )*
    };
}

impl_from_view! {
    (as_browser_view, BrowserView),
    (as_panel, Panel)
}

/// See [cef_view_delegate_t] for more documentation.
pub trait ViewDelegate: Sized {
    fn on_parent_view_changed(&self, _view: View, _added: bool, _parent: View) {}
    fn on_child_view_changed(&self, _view: View, _added: bool, _child: View) {}
    fn on_window_changed(&self, _view: View, _added: bool) {}

    fn into_raw(self) -> *mut cef_view_delegate_t {
        let mut object: cef_view_delegate_t = unsafe { std::mem::zeroed() };

        add_view_delegate_methods!(object);

        RcImpl::new(object, self).cast()
    }
}

/// View delegate could be other types' base. Use this macro to add view methods for them.
macro_rules! add_view_delegate_methods {
    ($name:ident) => {
        use crate::view::*;
        $name.on_parent_view_changed = Some(on_parent_view_changed::<Self>);
        $name.on_child_view_changed = Some(on_child_view_changed::<Self>);
        $name.on_window_changed = Some(on_window_changed::<Self>);
    };
}
pub(crate) use add_view_delegate_methods;

pub(crate) extern "C" fn on_parent_view_changed<I: ViewDelegate>(
    this: *mut cef_view_delegate_t,
    view: *mut cef_view_t,
    added: c_int,
    parent: *mut cef_view_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let view = View(unsafe { RefGuard::from_raw(view) });
    let added = added != 0;
    let parent = View(unsafe { RefGuard::from_raw(parent) });
    obj.interface.on_child_view_changed(view, added, parent);
}

pub(crate) extern "C" fn on_child_view_changed<I: ViewDelegate>(
    this: *mut cef_view_delegate_t,
    view: *mut cef_view_t,
    added: c_int,
    child: *mut cef_view_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let view = View(unsafe { RefGuard::from_raw(view) });
    let added = added != 0;
    let child = View(unsafe { RefGuard::from_raw(child) });
    obj.interface.on_child_view_changed(view, added, child);
}

pub(crate) extern "C" fn on_window_changed<I: ViewDelegate>(
    this: *mut cef_view_delegate_t,
    view: *mut cef_view_t,
    added: c_int,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let view = View(unsafe { RefGuard::from_raw(view) });
    let added = added != 0;
    obj.interface.on_window_changed(view, added);
}
