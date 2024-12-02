use super::ViewDelegate;
use crate::prelude::*;
use crate::{add_view_delegate_methods, string::CefString};

/// See [cef_button_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct Button(cef_button_t);

/// See [cef_label_button_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct LabelButton(cef_label_button_t);

pub trait ButtonDelegate: ViewDelegate {
    fn into_raw(self) -> *mut cef_sys::cef_button_delegate_t {
        let mut object: cef_sys::cef_button_delegate_t = unsafe { std::mem::zeroed() };
        let view = &mut object.base;
        add_view_delegate_methods!(view);
        RcImpl::new(object, self).cast()
    }
}

impl LabelButton {
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
            Ok(Self::from_raw(button))
        }
    }
}

/// See [cef_menu_button_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct MenuButton(cef_menu_button_t);

impl MenuButton {
    wrapper_methods!(
        /// See [cef_menu_button_t::show_menu]
        fn show_menu(
            &mut self,
            menu_model: crate::MenuModel,
            screen_point: &crate::Point,
            anchor_position: cef_sys::cef_menu_anchor_position_t,
        ) {
            self.0.show_menu.map(|f| unsafe {
                f(
                    self.0.get_this(),
                    menu_model.into_raw(),
                    std::ptr::from_ref(&screen_point),
                    anchor_position,
                )
            })
        }

        /// See [cef_menu_button_t::trigger_menu]
        fn trigger_menu(&mut self) {
            self.0.trigger_menu.map(|f| unsafe { f(self.0.get_this()) })
        }
    );
}

pub trait MenuButtonDelegate: ButtonDelegate {
    fn on_menu_button_pressed(
        &self,
        _menu_button: MenuButton,
        _screen_point: crate::Point,
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

impl MenuButton {
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
            Ok(Self::from_raw(button))
        }
    }
}

crate::convert_view! {
    (Button, as_label_button, LabelButton),
    (LabelButton, as_menu_button, MenuButton)
}
