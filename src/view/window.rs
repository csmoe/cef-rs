use crate::{
    add_view_delegate_methods,
    rc::{RcImpl, RefGuard},
    string::CefString,
    view::{Panel, PanelDelegate},
    wrapper, Rect, State,
};
use cef_sys::{
    cef_key_event_t, cef_runtime_style_t, cef_show_state_t, cef_window_create_top_level,
    cef_window_delegate_t, cef_window_info_t, cef_window_t,
};

/// See [cef_window_info_t] for more documentation.
#[derive(Debug, Default)]
pub struct WindowInfo {
    window_name: CefString,
    bounds: Rect,
    windowless_rendering_enabled: bool,
    shared_texture_enabled: bool,
    external_begin_frame_enabled: bool,
    #[cfg(target_os = "macos")]
    hidden: bool,
    #[cfg(target_os = "macos")]
    parent_view: Option<objc2_app_kit::NSView>,
    #[cfg(target_os = "macos")]
    view: Option<objc2_app_kit::NSView>,
    runtime_style: cef_sys::cef_runtime_style_t,
    #[cfg(windows)]
    menu: windows::Win32::UI::WindowsAndMessaging::HMENU,
    #[cfg(windows)]
    ex_style: u32,
    #[cfg(windows)]
    style: u32,
    #[cfg(any(windows, target_os = "linux"))]
    parent_window: windows::Win32::Foundation::HWND,
    #[cfg(any(windows, target_os = "linux"))]
    window: windows::Win32::Foundation::HWND,
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
            parent_view: self
                .parent_view
                .as_ref()
                .map(|v| core::ptr::from_ref(v).cast_mut().cast())
                .unwrap_or(core::ptr::null_mut()),
            #[cfg(target_os = "macos")]
            view: self
                .view
                .as_ref()
                .map(|v| core::ptr::from_ref(v).cast_mut().cast())
                .unwrap_or(core::ptr::null_mut()),
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
    pub fn close(&mut self);
    pub fn show(&mut self);
);

impl Window {
    pub fn create(delegate: impl WindowDelegate) -> crate::Result<Self> {
        let window = unsafe { cef_window_create_top_level(WindowDelegate::into_raw(delegate)) };
        if window.is_null() {
            return Err(crate::Error::NullPtr);
        }
        Ok(Window(unsafe { RefGuard::from_raw(window) }))
    }

    pub fn get_panel(&self) -> Panel {
        unsafe { Panel(self.0.convert()) }
    }
}

/// See [cef_window_delegate_t] for more documentation.
pub trait WindowDelegate: PanelDelegate {
    fn on_window_created(&self, _window: Window) {}
    fn on_window_closing(&self, _window: Window) {}
    fn on_window_destroyed(&self, _window: Window) {}

    fn on_window_activation_changed(&self, _window: Window, _activated: bool) {}

    fn on_window_bounds_changed(&self, _window: Window, _new_bounds: Rect) {}

    fn on_window_fullscreen_transition(&self, _window: Window, _fullscreen: bool) {}

    fn get_parent_window(&self, _window: Window, _is_menu: bool, _can_active_menu: bool) {}

    fn is_window_modal_dialog(&self, _window: Window) -> bool {
        false
    }

    fn get_initial_bounds(&self, _window: Window) -> Rect {
        todo!()
    }

    fn get_initial_show_state(&self, _window: Window) -> cef_show_state_t {
        todo!()
    }

    fn is_frameless(&self, _window: Window) -> bool {
        todo!()
    }

    fn with_standard_window_buttons(&self, _window: Window) -> bool {
        todo!()
    }

    fn get_titlebar_height(&self, _window: Window) -> i32 {
        todo!()
    }

    fn accepts_first_mouse(&self, _window: Window) -> State {
        todo!()
    }

    fn can_resize(&self, _window: Window) -> bool {
        true
    }

    fn can_minimize(&self, _window: Window) -> bool {
        true
    }

    fn can_maximize(&self, _window: Window) -> bool {
        true
    }

    fn can_close(&mut self, _window: Window) -> bool {
        true
    }

    fn on_accelerator(&self, _window: Window, _command_id: i32) -> bool {
        todo!()
    }

    fn on_key_event(&self, _window: Window, _event: cef_key_event_t) -> bool {
        false
    }

    fn on_theme_color_changed(&self, _window: Window, _chrome_theme: i32) {}

    fn get_window_runtime_style(&self) -> cef_runtime_style_t {
        todo!()
    }

    #[cfg(target_os = "linux")]
    fn get_linux_window_properties(
        &self,
        _window: Window,
        _properties: cef_sys::cef_linux_window_properties_t,
    ) -> bool {
        false
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
