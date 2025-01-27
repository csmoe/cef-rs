use crate::rc::Rc;
use crate::{
    prelude::*,
    rc::RcImpl,
    CefRect, CefSize,
};
use cef_sys::{cef_view_delegate_t, cef_view_t};
use cef_wrapper_macro::wrapper_methods;
use std::ffi::c_int;

mod panel;
pub use panel::*;
mod button;
pub use button::*;
mod browser;
pub use browser::*;
mod window;
pub use window::*;
mod textfield;
pub use textfield::*;
mod scroll_view;
pub use scroll_view::*;
mod layout;
pub use layout::*;

/// See [cef_view_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefView(cef_view_t);

impl CefView {
    //#[cfg(debug_assertions)]
    //pub fn get_type_string(&self) -> Option<CefString> {
    //    self.0
    //        .get_type_string
    //        .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.get_this())) })
    //}

    //pub fn to_string(&self, include_children: bool) -> Option<CefString> {
    //    to_string.and_then(|f| unsafe {
    //        CefString::from_userfree_cef(f(self.get_this(), include_children.into()))
    //    })
    //}

    //pub fn get_delegate(&self) -> Option<RefGuard<cef_view_delegate_t>> {
    //    get_delegate.and_then(|f| {
    //        let delegate = unsafe { f(self.get_this()) };
    //        if delegate.is_null() {
    //            None
    //        } else {
    //            Some(unsafe { RefGuard::from(delegate) })
    //        }
    //    })
    //}

    //pub fn get_window(&self) -> CefWindow {
    //    get_window.and_then(|f| {
    //        let window = f(self.get_this());
    //        if window.is_null() {
    //            None
    //        } else {
    //            Some(CefWindow::from(window))
    //        }
    //    })
    //}

    wrapper_methods!(
        /// See [cef_view_t::is_valid]
        fn is_valid(&self) -> bool;
        /// See [cef_view_t::is_attached]
        fn is_attached(&self) -> bool;
        /// See [cef_view_t::is_same]
        fn is_same(&self, other: CefView) -> bool;
        /// See [cef_view_t::get_id]
        fn get_id(&self) -> i32;
        /// See [cef_view_t::set_id]
        fn set_id(&mut self, id: i32);
        /// See [cef_view_t::get_group_id]
        fn get_group_id(&self) -> i32;
        /// See [cef_view_t::set_group_id]
        fn set_group_id(&mut self, group: i32);
        /// See [cef_view_t::get_parent_view]
        fn get_parent_view(&self) -> CefView {
            get_parent_view.and_then(|f| unsafe {
                let view = f(self.get_this());
                if view.is_null() {
                    None
                } else {
                    Some(CefView::from(view))
                }
            })
        }

        /// See [cef_view_t::as_panel]
        fn as_panel(&self) -> CefPanel {
            as_panel.map(|f| unsafe { CefPanel::from(f(self.get_this())) })
        }

        /// See [cef_view_t::as_browser_view]
        fn as_browser_view(&self) -> CefBrowserView {
            as_browser_view.map(|f| unsafe { CefBrowserView::from(f(self.get_this())) })
        }

        /// See [cef_view_t::as_textfield]
        fn as_textfield(&self) -> CefTextField {
            as_textfield.map(|f| unsafe { CefTextField::from(f(self.get_this())) })
        }

        /// See [cef_view_t::as_scroll_view]
        fn as_scroll_view(&self) -> CefScrollView {
            as_scroll_view.map(|f| unsafe { CefScrollView::from(f(self.get_this())) })
        }

        /// See [cef_view_t::as_button]
        fn as_button(&self) -> CefButton {
            as_button.map(|f| unsafe { CefButton::from(f(self.get_this())) })
        }

        /// See [cef_view_t::get_view_for_id]
        fn get_view_for_id(&self, id: i32) -> CefView {
            get_view_for_id.and_then(|f| unsafe {
                let view = f(self.get_this(), id);
                if view.is_null() {
                    None
                } else {
                    Some(CefView::from(view))
                }
            })
        }

        /// See [cef_view_t::get_bounds]
        fn get_bounds(&self) -> CefRect;
        /// See [cef_view_t::set_bounds]
        fn set_bounds(&mut self, bounds: &CefRect) {
            set_bounds.map(|f| unsafe { f(self.get_this(), bounds) })
        }

        /// See [cef_view_t::get_size]
        fn get_size(&self) -> CefSize;
        /// See [cef_view_t::set_size]
        fn set_size(&mut self, size: &CefSize) {
            set_size.map(|f| unsafe { f(self.get_this(), size) })
        }

        /// See [cef_view_t::get_position]
        fn get_position(&self) -> crate::CefPoint;
        /// See [cef_view_t::set_position]
        fn set_position(&mut self, position: &crate::CefPoint) {
            set_position.map(|f| unsafe { f(self.get_this(), position) })
        }

        /// See [cef_view_t::set_insets]
        fn set_insets(&mut self, inset: &crate::CefInsets) {
            set_insets.map(|f| unsafe { f(self.get_this(), inset) })
        }
        /// See [cef_view_t::get_insets]
        fn get_insets(&self) -> crate::CefInsets;

        /// See [cef_view_t::get_preferred_size]
        fn get_preferred_size(&self) -> CefSize;
        /// See [cef_view_t::size_to_preferred_size]
        fn size_to_preferred_size(&self);

        /// See [cef_view_t::get_minimum_size]
        fn get_minimum_size(&self) -> CefSize;
        /// See [cef_view_t::get_maximum_size]
        fn get_maximum_size(&self) -> CefSize;
        /// See [cef_view_t::get_height_for_width]
        fn get_height_for_width(&self, width: i32) -> i32;

        /// See [cef_view_t::invalidate_layout]
        fn invalidate_layout(&mut self);

        /// See [cef_view_t::set_visible]
        fn set_visible(&mut self, visible: bool);
        /// See [cef_view_t::is_visible]
        fn is_visible(&self) -> bool;

        /// See [cef_view_t::is_drawn]
        fn is_drawn(&self) -> bool;

        /// See [cef_view_t::set_focusable]
        fn set_focusable(&mut self, focus: bool);
        /// See [cef_view_t::is_focusable]
        fn is_focusable(&self) -> bool;

        /// See [cef_view_t::is_enabled]
        fn is_enabled(&self) -> bool;
        /// See [cef_view_t::set_enabled]
        fn set_enabled(&mut self, enabled: bool);

        /// See [cef_view_t::is_accessibility_focusable]
        fn is_accessibility_focusable(&self) -> bool;

        /// see [cef_view_t::request_focus]
        fn request_focus(&mut self);

        /// See [cef_view_t::set_background_color]
        fn set_background_color(&mut self, color: u32);
        /// See [cef_view_t::get_background_color]
        fn get_background_color(&self) -> u32;

        /// See [cef_view_t::get_theme_color]
        fn get_theme_color(&self, id: i32) -> u32;

        /// See [cef_view_t::convert_point_to_screen]
        fn convert_point_to_screen(&self, point: &mut crate::CefPoint) -> bool {
            convert_point_to_screen
                .map(|f| unsafe { f(self.get_this(), std::ptr::from_mut(point)) == 1 })
        }
        /// See [cef_view_t::convert_point_from_screen]
        fn convert_point_from_screen(&self, point: &mut crate::CefPoint) -> bool {
            convert_point_from_screen
                .map(|f| unsafe { f(self.get_this(), std::ptr::from_mut(point)) == 1 })
        }
        /// See [cef_view_t::convert_point_to_window]
        fn convert_point_to_window(&self, point: &mut crate::CefPoint) -> bool {
            convert_point_to_window
                .map(|f| unsafe { f(self.get_this(), std::ptr::from_mut(point)) == 1 })
        }
        /// See [cef_view_t::convert_point_from_window]
        fn convert_point_from_window(&self, point: &mut crate::CefPoint) -> bool {
            convert_point_from_window
                .map(|f| unsafe { f(self.get_this(), std::ptr::from_mut(point)) == 1 })
        }
        /// See [cef_view_t::convert_point_to_view]
        fn convert_point_to_view(&self, point: &mut crate::CefPoint, view: CefView) -> bool {
            convert_point_to_view.map(|f| unsafe {
                f(self.get_this(), view.get_this(), std::ptr::from_mut(point)) == 1
            })
        }
        /// See [cef_view_t::convert_point_from_view]
        fn convert_point_from_view(&self, point: &mut crate::CefPoint, view: CefView) -> bool {
            convert_point_from_view.map(|f| unsafe {
                f(self.get_this(), view.get_this(), std::ptr::from_mut(point)) == 1
            })
        }
    );
}

impl Rc for cef_view_delegate_t {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        &self.base
    }
}

/// See [cef_view_delegate_t] for more documentation.
pub trait ViewDelegate: Sized {
    /// See [cef_view_delegate_t::on_parent_view_changed]
    fn on_parent_view_changed(&self, _view: CefView, _added: bool, _parent: CefView) {}
    /// See [cef_view_delegate_t::on_child_view_changed]
    fn on_child_view_changed(&self, _view: CefView, _added: bool, _child: CefView) {}
    /// See [cef_view_delegate_t::on_window_changed]
    fn on_window_changed(&self, _view: CefView, _added: bool) {}
    /// See [cef_view_delegate_t::on_layout_changed]
    fn on_layout_changed(&self, _view: CefView, _new_bounds: CefRect) {}
    /// See [cef_view_delegate_t::on_focus]
    fn on_focus(&self, _view: CefView) {}
    /// See [cef_view_delegate_t::on_blur]
    fn on_blur(&self, _view: CefView) {}
    /// See [cef_view_delegate_t::on_theme_changed]
    fn on_theme_changed(&self, _view: CefView) {}
    /// See [cef_view_delegate_t::get_preferred_size]
    fn get_preferred_size(&self, _view: CefView) -> CefSize {
        todo!()
    }
    /// See [cef_view_delegate_t::get_minimum_size]
    fn get_minimum_size(&self, _view: CefView) -> CefSize {
        todo!()
    }
    /// See [cef_view_delegate_t::get_maximum_size]
    fn get_maximum_size(&self, _view: CefView) -> CefSize {
        todo!()
    }
    /// See [cef_view_delegate_t::get_height_for_width]
    fn get_height_for_width(&self, _view: CefView, _width: i32) -> i32 {
        todo!()
    }

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
    let view = CefView::from(view);
    let added = added != 0;
    let parent = CefView::from(parent);
    obj.interface.on_child_view_changed(view, added, parent);
}

pub(crate) extern "C" fn on_child_view_changed<I: ViewDelegate>(
    this: *mut cef_view_delegate_t,
    view: *mut cef_view_t,
    added: c_int,
    child: *mut cef_view_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let view = CefView::from(view);
    let added = added != 0;
    let child = CefView::from(child);
    obj.interface.on_child_view_changed(view, added, child);
}

pub(crate) extern "C" fn on_window_changed<I: ViewDelegate>(
    this: *mut cef_view_delegate_t,
    view: *mut cef_view_t,
    added: c_int,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let view = CefView::from(view);
    let added = added != 0;
    obj.interface.on_window_changed(view, added);
}
