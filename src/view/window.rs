use crate::prelude::*;
use crate::{
    add_view_delegate_methods,
    rc::RcImpl,
    string::CefString,
    view::PanelDelegate,
    CefRect, CefState,
};

/// See [cef_window_info_t] for more documentation.
#[derive(Debug, Default)]
pub struct CefWindowInfo {
    /// See [cef_window_info_t::window_name]
    pub window_name: CefString,
    /// See [cef_window_info_t::bounds]
    pub bounds: CefRect,
    /// See [cef_window_info_t::windowless_rendering_enabled]
    pub windowless_rendering_enabled: bool,
    /// See [cef_window_info_t::shared_texture_enabled]
    pub shared_texture_enabled: bool,
    /// See [cef_window_info_t::external_begin_frame_enabled]
    pub external_begin_frame_enabled: bool,

    #[cfg(target_os = "macos")]
    /// See [cef_window_info_t::hidden]
    pub hidden: bool,
    #[cfg(target_os = "macos")]
    /// See [cef_window_info_t::parent_view]
    pub parent_view: Option<objc2::rc::Retained<objc2_app_kit::NSView>>,
    #[cfg(target_os = "macos")]
    /// See [cef_window_info_t::view]
    pub view: Option<objc2::rc::Retained<objc2_app_kit::NSView>>,
    /// See [cef_window_info_t::runtime_style]
    pub runtime_style: cef_sys::cef_runtime_style_t,
    #[cfg(windows)]
    /// See [cef_window_info_t::menu]
    pub menu: windows::Win32::UI::WindowsAndMessaging::HMENU,
    #[cfg(windows)]
    /// Seee [cef_window_info_t::ex_style]
    pub ex_style: u32,
    #[cfg(windows)]
    /// See [cef_window_info_t::style]
    pub style: u32,
    #[cfg(any(windows, target_os = "linux"))]
    /// See [cef_window_info_t::parent_window]
    pub parent_window: windows::Win32::Foundation::HWND,
    #[cfg(any(windows, target_os = "linux"))]
    /// See [cef_window_info_t::window]
    pub window: windows::Win32::Foundation::HWND,
}

impl CefWindowInfo {
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

    pub unsafe fn from_raw(raw: *mut cef_window_info_t) -> Option<Self> {
        if raw.is_null() {
            return None;
        };
        let cef_window_info_t {
            window_name,
            bounds,
            #[cfg(target_os = "macos")]
            hidden,
            #[cfg(target_os = "macos")]
            parent_view,
            windowless_rendering_enabled,
            shared_texture_enabled,
            external_begin_frame_enabled,
            #[cfg(target_os = "macos")]
            view,
            runtime_style,
            #[cfg(windows)]
            menu,
            #[cfg(windows)]
            ex_style,
            #[cfg(windows)]
            style,
            #[cfg(any(windows, target_os = "linux"))]
            parent_window,
            #[cfg(any(windows, target_os = "linux"))]
            window,
        } = *raw;
        let windown_info = Self {
            window_name: CefString::from_raw(&window_name).unwrap_or_default(),
            bounds,
            #[cfg(target_os = "macos")]
            hidden: hidden == 1,
            #[cfg(target_os = "macos")]
            parent_view: objc2::rc::Id::retain(parent_view.cast() as _),
            windowless_rendering_enabled: windowless_rendering_enabled == 1,
            shared_texture_enabled: shared_texture_enabled == 1,
            external_begin_frame_enabled: external_begin_frame_enabled == 1,
            #[cfg(target_os = "macos")]
            view: objc2::rc::Id::retain(view.cast() as _),
            runtime_style,
            #[cfg(windows)]
            menu: windows::Win32::UI::WindowsAndMessaging::HMENU(menu.cast()),
            #[cfg(windows)]
            ex_style,
            #[cfg(windows)]
            style,
            #[cfg(windows)]
            parent_window: windows::Win32::Foundation::HWND(parent_window.cast()),
            #[cfg(target_os = "linux")]
            parent_window: parent_window.cast(),
            #[cfg(windows)]
            window: windows::Win32::Foundation::HWND(window.cast()),
            #[cfg(target_os = "linux")]
            window: window.cast(),
        };
        Some(windown_info)
    }
}

/// See [cef_window_t] for more documentation.
#[derive(Clone)]
#[wrapper]
pub struct CefWindow(cef_window_t);

impl CefWindow {
    wrapper_methods!(
        /// See [cef_window_t::close]
        fn close(&mut self);
        /// See [cef_window_t::show]
        fn show(&mut self);
        /// See [cef_window_t::show_as_browser_modal_dialog]
        fn show_as_browser_modal_dialog(&mut self, browser_view: crate::CefBrowserView);
        /// See [cef_window_t::hide]
        fn hide(&mut self);
        /// See [cef_window_t::center_window]
        fn center_window(&mut self, size: *const cef_size_t);
        /// See [cef_window_t::is_closed]
        fn is_closed(&self) -> bool;
        /// See [cef_window_t::activate]
        fn activate(&mut self);
        /// See [cef_window_t::deactivate]
        fn deactivate(&mut self);
        /// See [cef_window_t::is_active]
        fn is_active(&self) -> bool;
        /// See [cef_window_t::bring_to_top]
        fn bring_to_top(&mut self);
        /// See [cef_window_t::set_always_on_top]
        fn set_always_on_top(&mut self, on_top: i32);
        /// See [cef_window_t::is_always_on_top]
        fn is_always_on_top(&self) -> bool;
        /// See [cef_window_t::maximize]
        fn maximize(&mut self);
        /// See [cef_window_t::minimize]
        fn minimize(&mut self);
        /// See [cef_window_t::restore]
        fn restore(&mut self);
        /// See [cef_window_t::set_fullscreen]
        fn set_fullscreen(&mut self, fullscreen: bool);
        /// See [cef_window_t::is_maximized]
        fn is_maximized(&self) -> bool;
        /// See [cef_window_t::is_minimized]
        fn is_minimized(&self) -> bool;
        /// See [cef_window_t::is_fullscreen]
        fn is_fullscreen(&self) -> bool;
        /// See [cef_window_t::set_title]
        fn set_title(&mut self, title: &str) {
            set_title.map(|f| unsafe { f(self.get_this(), &CefString::from(title).as_raw()) })
        }
        /// See [cef_window_t::get_title]
        fn get_title(&self) -> CefString {
            get_title.and_then(|f| unsafe { CefString::from_userfree_cef(f(self.get_this())) })
        }
        /// See [cef_window_t::set_window_icon]
        fn set_window_icon(&mut self, image: *mut cef_image_t);
        /// See [cef_window_t::get_window_icon]
        fn get_window_icon(&self) -> *mut cef_image_t;
        /// See [cef_window_t::set_window_app_icon]
        fn set_window_app_icon(&mut self, image: *mut cef_image_t);
        /// See [cef_window_t::get_window_app_icon]
        fn get_window_app_icon(&self) -> cef_sys::cef_image_t {
            todo!()
        }
        /// See [cef_window_t::add_overlay_view]
        fn add_overlay_view(
            &mut self,
            _view: *mut cef_view_t,
            _docking_mode: cef_docking_mode_t,
            _can_activate: i32,
        ) -> cef_sys::cef_overlay_controller_t {
            todo!()
        }
        /// See [cef_window_t::show_menu]
        fn show_menu(
            &mut self,
            _menu_model: *mut cef_menu_model_t,
            _screen_point: cef_point_t,
            _anchor_position: cef_menu_anchor_position_t,
        ) {
            todo!()
        }
        /// See [cef_window_t::cancel_menu]
        fn cancel_menu(&mut self);
        /// See [cef_window_t::get_display]
        fn get_display(&self) -> *mut cef_sys::cef_display_t;
        /// See [cef_window_t::get_client_area_bounds_in_screen]
        fn get_client_area_bounds_in_screen(&self) -> cef_rect_t;
        /// See [cef_window_t::set_draggable_regions]
        fn set_draggable_regions(
            &mut self,
            regions_count: usize,
            regions: *const cef_draggable_region_t,
        );
        #[cfg(target_family = "unix")]
        /// See [cef_window_t::get_window_handle]
        fn get_window_handle(&self) -> *mut ::std::os::raw::c_void;
        #[cfg(target_os = "windows")]
        /// See [cef_window_t::get_window_handle]
        fn get_window_handle(&self) -> *mut windows::Win32::Foundation::HWND {
            get_window_handle.map(|f| unsafe { f(self.get_this()).cast() })
        }
        /// See [cef_window_t::send_key_press]
        fn send_key_press(&mut self, key_code: i32, event_flags: u32);
        /// See [cef_window_t::send_mouse_move]
        fn send_mouse_move(&mut self, screen_x: i32, screen_y: i32);
        /// See [cef_window_t::send_mouse_events]
        fn send_mouse_events(
            &mut self,
            button: cef_mouse_button_type_t,
            mouse_down: bool,
            mouse_up: bool,
        );
        /// See [cef_window_t::set_accelerator]
        fn set_accelerator(
            &mut self,
            command_id: i32,
            key_code: i32,
            shift_pressed: bool,
            ctrl_pressed: bool,
            alt_pressed: bool,
            high_priority: bool,
        );
        /// See [cef_window_t::remove_accelerator]
        fn remove_accelerator(&mut self, command_id: i32);
        /// See [cef_window_t::remove_all_accelerators]
        fn remove_all_accelerators(&mut self);
        /// See [cef_window_t::set_theme_color]
        fn set_theme_color(&mut self, color_id: i32, color: cef_color_t);
        /// See [cef_window_t::theme_changed]
        fn theme_changed(&mut self);
        /// See [cef_window_t::get_runtime_style]
        fn get_runtime_style(&self) -> cef_runtime_style_t;
    );
}

impl CefWindow {
    /// See [cef_window_create_top_level]
    pub fn create(delegate: impl WindowDelegate) -> Result<Self> {
        let window = unsafe { cef_window_create_top_level(WindowDelegate::into_raw(delegate)) };
        if window.is_null() {
            return Err(Error::NullPtr);
        }
        Ok(CefWindow::from(window))
    }
}

/// See [cef_window_delegate_t] for more documentation.
pub trait WindowDelegate: PanelDelegate {
    /// See [cef_window_delegate_t::on_window_created]
    fn on_window_created(&self, _window: CefWindow) {}
    /// See [cef_window_delegate_t::on_window_closing]
    fn on_window_closing(&self, _window: CefWindow) {}
    /// See [cef_window_delegate_t::on_window_destroyed]
    fn on_window_destroyed(&self, _window: CefWindow) {}

    /// See [cef_window_delegate_t::on_window_activation_changed]
    fn on_window_activation_changed(&self, _window: CefWindow, _activated: bool) {}

    /// See [cef_window_delegate_t::on_window_bounds_changed]
    fn on_window_bounds_changed(&self, _window: CefWindow, _new_bounds: CefRect) {}

    /// See [cef_window_delegate_t::on_window_fullscreen_transition]
    fn on_window_fullscreen_transition(&self, _window: CefWindow, _fullscreen: bool) {}

    /// See [cef_window_delegate_t::get_parent_window]
    fn get_parent_window(&self, _window: CefWindow, _is_menu: bool, _can_active_menu: bool) {}

    /// See [cef_window_delegate_t::is_window_modal_dialog]
    fn is_window_modal_dialog(&self, _window: CefWindow) -> bool {
        false
    }

    /// See [cef_window_delegate_t::get_initial_bounds]
    fn get_initial_bounds(&self, _window: CefWindow) -> CefRect {
        todo!()
    }

    /// See [cef_window_delegate_t::get_initial_show_state]
    fn get_initial_show_state(&self, _window: CefWindow) -> cef_show_state_t {
        todo!()
    }

    /// See [cef_window_delegate_t::is_frameless]
    fn is_frameless(&self, _window: CefWindow) -> bool {
        todo!()
    }

    /// See [cef_window_delegate_t::with_standard_window_buttons]
    fn with_standard_window_buttons(&self, _window: CefWindow) -> bool {
        todo!()
    }

    /// See [cef_window_delegate_t::get_titlebar_height]
    fn get_titlebar_height(&self, _window: CefWindow) -> i32 {
        todo!()
    }

    /// See [cef_window_delegate_t::accepts_first_mouse]
    fn accepts_first_mouse(&self, _window: CefWindow) -> CefState {
        todo!()
    }

    /// See [cef_window_delegate_t::can_resize]
    fn can_resize(&self, _window: CefWindow) -> bool {
        true
    }

    /// See [cef_window_delegate_t::can_minimize]
    fn can_minimize(&self, _window: CefWindow) -> bool {
        true
    }

    /// see [cef_window_delegate_t::can_maximize]
    fn can_maximize(&self, _window: CefWindow) -> bool {
        true
    }

    /// See [cef_window_delegate_t::can_close]
    fn can_close(&mut self, _window: CefWindow) -> bool {
        true
    }

    /// See [cef_window_delegate_t::on_accelerator]
    fn on_accelerator(&self, _window: CefWindow, _command_id: i32) -> bool {
        todo!()
    }

    /// See [cef_window_delegate_t::on_key_event]
    fn on_key_event(&self, _window: CefWindow, _event: cef_key_event_t) -> bool {
        false
    }

    /// See [cef_window_delegate_t::on_theme_colors_changed]
    fn on_theme_colors_changed(&self, _window: CefWindow, _chrome_theme: i32) {}

    /// See [cef_window_delegate_t::get_window_runtime_style]
    fn get_window_runtime_style(&self) -> cef_runtime_style_t {
        todo!()
    }

    #[cfg(target_os = "linux")]
    /// See [cef_window_delegate_t::get_linux_window_properties]
    fn get_linux_window_properties(
        &self,
        _window: CefWindow,
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
    let window = CefWindow::from(window);
    obj.interface.on_window_created(window);
}

extern "C" fn on_window_closing<I: WindowDelegate>(
    this: *mut cef_window_delegate_t,
    window: *mut cef_window_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let window = CefWindow::from(window);
    obj.interface.on_window_closing(window);
}

extern "C" fn on_window_destroyed<I: WindowDelegate>(
    this: *mut cef_window_delegate_t,
    window: *mut cef_window_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let window = CefWindow::from(window);
    obj.interface.on_window_destroyed(window);
}

extern "C" fn can_close<I: WindowDelegate>(
    this: *mut cef_window_delegate_t,
    window: *mut cef_window_t,
) -> i32 {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let window = CefWindow::from(window);
    let result = obj.interface.can_close(window);
    result as i32
}
