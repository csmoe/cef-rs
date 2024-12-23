use super::ViewDelegate;
use crate::prelude::*;
use crate::{add_view_delegate_methods, string::CefString};

/// See [cef_button_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefButton(cef_button_t);

impl CefButton {
    wrapper_methods! {
        /// See [cef_button_t::as_label_button]
        fn as_label_button(&self) -> CefLabelButton {
            as_label_button.map(|f| unsafe{
                CefLabelButton::from(f(self.get_this()))
            })
        }

    }
}

/// See [cef_label_button_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefLabelButton(cef_label_button_t);

pub trait ButtonDelegate: ViewDelegate {
    fn into_raw(self) -> *mut cef_sys::cef_button_delegate_t {
        let mut object: cef_sys::cef_button_delegate_t = unsafe { std::mem::zeroed() };
        let view = &mut object.base;
        add_view_delegate_methods!(view);
        RcImpl::new(object, self).cast()
    }
}

impl CefLabelButton {
    pub fn create(delegate: impl ButtonDelegate, text: CefString) -> Result<Self> {
        unsafe {
            // TODO: ui thread restriction
            let button = cef_sys::cef_label_button_create(
                <_ as ButtonDelegate>::into_raw(delegate),
                std::ptr::from_ref(&text.as_raw()),
            );
            if button.is_null() {
                return Err(Error::NullPtr);
            }
            Ok(Self::from(button))
        }
    }

    wrapper_methods! {
        /// See [cef_lable_button_t::as_menu_button]
        fn as_menu_button(&self) -> CefMenuButton {
             as_menu_button.map(|f| unsafe{
                CefMenuButton::from(f(self.get_this()))
            })
        }
    }
}

/// See [cef_menu_button_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefMenuButton(cef_menu_button_t);

impl CefMenuButton {
    wrapper_methods!(
        /// See [cef_menu_button_t::show_menu]
        fn show_menu(
            &mut self,
            menu_model: crate::CefMenuModel,
            screen_point: &crate::CefPoint,
            anchor_position: cef_sys::cef_menu_anchor_position_t,
        ) {
            show_menu.map(|f| unsafe {
                f(
                    self.get_this(),
                    menu_model.into_raw(),
                    screen_point,
                    anchor_position,
                )
            })
        }

        /// See [cef_menu_button_t::trigger_menu]
        fn trigger_menu(&mut self) {
            trigger_menu.map(|f| unsafe { f(self.get_this()) })
        }
    );
}

pub trait MenuButtonDelegate: ButtonDelegate {
    fn on_menu_button_pressed(
        &self,
        _menu_button: CefMenuButton,
        _screen_point: crate::CefPoint,
        _button_pressed_lock: cef_sys::cef_menu_button_pressed_lock_t,
    ) {
    }

    fn into_raw(self) -> *mut cef_sys::cef_menu_button_delegate_t {
        let mut object: cef_sys::cef_menu_button_delegate_t = unsafe { std::mem::zeroed() };
        let view = &mut object.base.base;
        // TODO: add button methods
        add_view_delegate_methods!(view);
        RcImpl::new(object, self).cast()
    }
}

impl CefMenuButton {
    /// See [cef_sys::cef_menu_button_create]
    pub fn create(delegate: impl MenuButtonDelegate, text: CefString) -> Result<Self> {
        unsafe {
            let button = cef_sys::cef_menu_button_create(
                MenuButtonDelegate::into_raw(delegate),
                core::ptr::from_ref(&text.as_raw()),
            );
            if button.is_null() {
                return Err(Error::NullPtr);
            }
            Ok(Self::from(button))
        }
    }
}
