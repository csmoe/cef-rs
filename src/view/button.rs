use super::ViewDelegate;
use crate::{add_view_delegate_methods, string::CefString};
use cef_sys::{cef_button_t, cef_label_button_t, cef_menu_button_t};
use cef_wrapper_macro::wrapper_methods;

crate::wrapper!(
    /// See [cef_button_t] for more documentation.
    #[derive(Debug, Clone)]
    pub struct Button(cef_button_t);
);

crate::wrapper!(
    /// See [cef_label_button_t] for more documentation.
    #[derive(Debug, Clone)]
    pub struct LabelButton(cef_label_button_t);
);

pub trait ButtonDelegate: ViewDelegate {
    fn into_raw(self) -> *mut cef_sys::cef_button_delegate_t {
        let mut object: cef_sys::cef_button_delegate_t = unsafe { std::mem::zeroed() };
        let view = &mut object.base;
        add_view_delegate_methods!(view);
        RcImpl::new(object, self).cast()
    }
}

impl LabelButton {
    pub fn create(delegate: impl ButtonDelegate, text: CefString) -> crate::Result<Self> {
        unsafe {
            // TODO: ui thread restriction
            let button = cef_sys::cef_label_button_create(
                <_ as ButtonDelegate>::into_raw(delegate),
                std::ptr::from_ref(&text.as_raw()),
            );
            if button.is_null() {
                return Err(crate::Error::NullPtr);
            }
            Ok(Self::from_raw(button))
        }
    }
}

crate::wrapper!(
    /// See [cef_menu_button_t] for more documentation.
    #[derive(Debug, Clone)]
    pub struct MenuButton(cef_menu_button_t);
);

impl MenuButton {
    wrapper_methods!(
        /// See [cef_menu_button_t::show_menu]
        fn show_menu(
            &mut self,
            menu_model: *mut cef_sys::cef_menu_model_t,
            screen_point: &crate::Point,
            anchor_position: cef_sys::cef_menu_anchor_position_t,
        ) {
            self.0.show_menu.map(|f| unsafe {
                f(
                    self.0.get_this(),
                    menu_model,
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
        _screen_point: cef_sys::cef_point_t,
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
    pub fn create(delegate: impl MenuButtonDelegate, text: CefString) -> crate::Result<Self> {
        unsafe {
            let button = cef_sys::cef_menu_button_create(
                MenuButtonDelegate::into_raw(delegate),
                core::ptr::from_ref(&text.as_raw()),
            );
            if button.is_null() {
                return Err(crate::Error::NullPtr);
            }
            Ok(Self::from_raw(button))
        }
    }
}

crate::convert_view! {
    (Button, as_label_button, LabelButton),
    (LabelButton, as_menu_button, MenuButton)
}
