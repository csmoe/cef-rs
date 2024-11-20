use crate::{
    add_view_delegate_methods,
    rc::{RcImpl, RefGuard},
    string::CefString,
    view::{Panel, PanelDelegate},
    wrapper, Rect, State,
};
use cef_sys::{
    cef_key_event_t, cef_runtime_style_t, cef_show_state_t, cef_window_create_top_level,
    cef_window_delegate_t, cef_window_info_t, cef_window_t, *,
};
use cef_wrapper_macro::wrapper_methods;

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

wrapper! {
    /// See [cef_window_t] for more documentation.
    #[derive(Debug, Clone)]
    pub struct Window(cef_window_t);
}

impl Window {
    wrapper_methods! {
        pub fn close(&mut self) {}
        pub fn show(&mut self) {}
        pub fn show_as_browser_modal_dialog(&mut self, browser_view: crate::BrowserView) {}
        pub fn hide(&mut self) {}
        pub fn center_window(&mut self, size: &cef_size_t) {}
        pub fn is_closed(&self) {}
        pub fn activate(&mut self) {}
        pub fn deactivate(&mut self) {}
        pub fn is_active(&self) -> bool {}
        pub fn bring_to_top(&mut self) {}
        pub fn set_always_on_top(&mut self, on_top: bool) {}
        pub fn is_always_on_top(&self) -> bool {}
        pub fn maximize(&mut self) {}
        pub fn minimize(&mut self) {}
        pub fn restore(&mut self) {}
        pub fn set_fullscreen(&mut self, fullscreen: bool) {}
        pub fn is_maximized(&self) -> bool {}
        pub fn is_minimized(&self) -> bool {}
        pub fn is_fullscreen(&self) -> bool {}
        pub fn set_title(&mut self, title: &str) {}
        pub fn get_title(&self) -> CefString {}
        pub fn set_window_icon(&mut self, image: cef_image_t) {}
        pub fn get_window_icon(&self) -> cef_image_t {}
        pub fn set_window_app_icon(&mut self, image: cef_sys::cef_image_t) {}
        pub fn get_window_app_icon(&self) -> cef_sys::cef_image_t {}
        pub fn add_overlay_view(&mut self, view: crate::View, docking_mode: cef_docking_mode_t, can_activate: bool) -> cef_sys::cef_overlay_controller_t{}
        pub fn show_menu(&mut self, menu_model: cef_menu_model_t, screen_point: &cef_point_t, anchor_position: cef_menu_anchor_position_t) {}
        pub fn cancel_menu(&mut self){}
        pub fn get_display(&self) -> cef_sys::cef_display_t {}
        pub fn get_client_area_bounds_in_screen(&self) -> cef_rect_t {}
        pub fn set_draggable_regions(&mut self, regions: &[cef_draggable_region_t]) {}
        pub fn get_window_handle(&self) -> *mut ::std::os::raw::c_void {}
        pub fn send_key_press(&mut self, key_code: i32, event_flags: u32) {}
        pub fn send_mouse_move(&mut self, screen_x: i32, screen_y: i32) {}
        pub fn send_mouse_events(&mut self, button: cef_mouse_button_type_t, mouse_down: bool, mouse_up: bool) {}
        pub fn set_accelerator(&mut self, command_id: i32, key_code: i32, shift_pressed: bool, ctrl_pressed: bool, alt_pressed: bool, high_priority: bool) {}
        pub fn remove_accelerator(&mut self, command_id: i32) {}
        pub fn remove_all_accelerators(&mut self) {}
        pub fn set_theme_color(&mut self, color_id: i32, color: cef_color_t) {}
        pub fn theme_changed(&mut self) {}
        pub fn get_runtime_style(&self) -> cef_runtime_style_t {}
    }
}

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
