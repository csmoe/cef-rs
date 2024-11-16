use cef_sys::{
    cef_window_create_top_level, cef_window_delegate_t, cef_window_info_t, cef_window_t,
};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::HMENU;

use crate::{
    add_view_delegate_methods,
    view::{Panel, PanelDelegate},
    rc::{RcImpl, RefGuard},
    string::CefString,
    wrapper, Rect,
};

/// See [cef_window_info_t] for more documentation.
#[derive(Debug, Clone, Default)]
pub struct WindowInfo {
    window_name: CefString,
    bounds: Rect,
    windowless_rendering_enabled: bool,
    shared_texture_enabled: bool,
    external_begin_frame_enabled: bool,
    #[cfg(target_os = "macos")]
    hidden: bool,
    #[cfg(target_os = "macos")]
    parent_view: *mut std::ffi::c_void,
    #[cfg(target_os = "macos")]
    view: *mut std::ffi::c_void,
    runtime_style: cef_sys::cef_runtime_style_t,
    #[cfg(windows)]
    menu: HMENU,
    #[cfg(windows)]
    ex_style: u32,
    #[cfg(windows)]
    style: u32,
    #[cfg(any(windows, target_os = "linux"))]
    parent_window: HWND,
    #[cfg(any(windows, target_os = "linux"))]
    window: HWND,
}

impl WindowInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_raw(&self) -> cef_window_info_t {
        cef_window_info_t {
            window_name: self.window_name.as_raw(),
            bounds: self.bounds,
            windowless_rendering_enabled: self.windowless_rendering_enabled.into(),
            shared_texture_enabled: self.shared_texture_enabled.into(),
            external_begin_frame_enabled: self.external_begin_frame_enabled.into(),
            #[cfg(target_os = "macos")]
            hidden: self.hidden.into(),
            #[cfg(target_os = "macos")]
            parent_view: self.view.cast(),
            #[cfg(target_os = "macos")]
            view: self.view.cast(),
            runtime_style: self.runtime_style,
            #[cfg(windows)]
            menu: self.menu.0.cast(),
            #[cfg(windows)]
            ex_style: self.ex_style,
            #[cfg(windows)]
            style: self.style,
            #[cfg(any(windows, target_os = "linux"))]
            parent_window: self.parent_window.0.cast(),
            #[cfg(any(windows, target_os = "linux"))]
            window: self.window.0.cast(),
        }
    }
}

wrapper!(
    #[doc = "See [cef_window_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct Window(cef_window_t);
    pub fn close(&self);
    pub fn show(&self);
);

impl Window {
    pub fn get_panel(&self) -> Panel {
        unsafe { Panel(self.0.convert()) }
    }
}

/// See [cef_window_delegate_t] for more documentation.
pub trait WindowDelegate: PanelDelegate {
    fn on_window_created(&self, _window: Window) {}
    fn on_window_closing(&self, _window: Window) {}
    fn on_window_destroyed(&self, _window: Window) {}
    fn can_close(&mut self, _window: Window) -> bool {
        true
    }

    fn into_raw(self) -> *mut cef_window_delegate_t {
        let mut object: cef_window_delegate_t = unsafe { std::mem::zeroed() };

        // Panal delegate doesn't have any methods. So we skip to view.
        let view = &mut object.base.base;
        add_view_delegate_methods!(view);

        object.on_window_created = Some(on_window_created::<Self>);
        object.on_window_closing = Some(on_window_closing::<Self>);
        object.on_window_destroyed = Some(on_window_destroyed::<Self>);
        object.can_close = Some(can_close::<Self>);

        RcImpl::new(object, self).cast()
    }
}

extern "C" fn on_window_created<I: WindowDelegate>(
    this: *mut cef_window_delegate_t,
    window: *mut cef_window_t,
) {
    let obj: &RcImpl<_, I> = RcImpl::get(this);
    let window = Window(unsafe { RefGuard::from_raw(window) });
    obj.interface.on_window_created(window);
}

extern "C" fn on_window_closing<I: WindowDelegate>(
    this: *mut cef_window_delegate_t,
    window: *mut cef_window_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let window = Window(unsafe { RefGuard::from_raw(window) });
    obj.interface.on_window_closing(window);
}

extern "C" fn on_window_destroyed<I: WindowDelegate>(
    this: *mut cef_window_delegate_t,
    window: *mut cef_window_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let window = Window(unsafe { RefGuard::from_raw(window) });
    obj.interface.on_window_destroyed(window);
}

extern "C" fn can_close<I: WindowDelegate>(
    this: *mut cef_window_delegate_t,
    window: *mut cef_window_t,
) -> i32 {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let window = Window(unsafe { RefGuard::from_raw(window) });
    let result = obj.interface.can_close(window);
    result as i32
}

pub fn create_top_level_window(delegate: impl WindowDelegate) -> Window {
    let window = unsafe { cef_window_create_top_level(WindowDelegate::into_raw(delegate)) };
    Window(unsafe { RefGuard::from_raw(window) })
}
