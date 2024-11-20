use super::ViewDelegate;
use crate::{add_view_delegate_methods, rc::RefGuard, string::CefString};
use cef_sys::{cef_button_t, cef_label_button_t, cef_menu_button_t};

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
            Ok(Self(RefGuard::from_raw(button)))
        }
    }
}

crate::wrapper!(
    /// See [cef_menu_button_t] for more documentation.
    #[derive(Debug, Clone)]
    pub struct MenuButton(cef_menu_button_t);
);

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
    pub fn create(delegate: impl MenuButtonDelegate, text: CefString) -> crate::Result<Self> {
        unsafe {
            let button = cef_sys::cef_menu_button_create(
                MenuButtonDelegate::into_raw(delegate),
                core::ptr::from_ref(&text.as_raw()),
            );
            if button.is_null() {
                return Err(crate::Error::NullPtr);
            }
            Ok(Self(RefGuard::from_raw(button)))
        }
    }
}

crate::convert_view! {
    (Button, as_label_button, LabelButton),
    (LabelButton, as_menu_button, MenuButton)
}
