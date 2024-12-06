use crate::{
    client::Client, error::Result, prelude::*, string::CefString, view::WindowInfo, BrowserView,
    ChromeToolbarType, State,
};
use cef_sys::{
    cef_browser_host_create_browser_sync, cef_browser_settings_t, cef_browser_t, cef_client_t,
    cef_gesture_command_t,
};
use std::{ffi::c_int, ptr::null_mut};

pub mod frame;

/// See [cef_browser_settings_t] for more documentation.
#[derive(Debug, Clone)]
pub struct BrowserSettings {
    /// See [cef_browser_settings_t::windowless_frame_rate]
    pub windowless_frame_rate: usize,
    pub standard_font_family: CefString,
    pub fixed_font_family: CefString,
    pub serif_font_family: CefString,
    pub sans_serif_font_family: CefString,
    pub cursive_font_family: CefString,
    pub fantasy_font_family: CefString,
    pub default_font_size: u32,
    pub default_fixed_font_size: u32,
    pub minimum_font_size: u32,
    pub minimum_logical_font_size: u32,
    pub default_encoding: CefString,
    pub remote_fonts: State,
    pub javascript: State,
    pub javascript_close_windows: State,
    pub javascript_access_clipboard: State,
    pub javascript_dom_paste: State,
    pub image_loading: State,
    pub image_shrink_standalone_to_fit: State,
    pub text_area_resize: State,
    pub tab_to_links: State,
    pub local_storage: State,
    pub databases: State,
    pub webgl: State,
    pub background_color: u32,
    pub chrome_zoom_bubble: State,
    pub chrome_status_bubble: State,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            windowless_frame_rate: Default::default(),
            standard_font_family: Default::default(),
            fixed_font_family: Default::default(),
            serif_font_family: Default::default(),
            sans_serif_font_family: Default::default(),
            cursive_font_family: Default::default(),
            fantasy_font_family: Default::default(),
            default_font_size: Default::default(),
            default_fixed_font_size: Default::default(),
            minimum_font_size: Default::default(),
            minimum_logical_font_size: Default::default(),
            default_encoding: Default::default(),
            remote_fonts: State::STATE_DEFAULT,
            javascript: State::STATE_DEFAULT,
            javascript_close_windows: State::STATE_DEFAULT,
            javascript_access_clipboard: State::STATE_DEFAULT,
            javascript_dom_paste: State::STATE_DEFAULT,
            image_loading: State::STATE_DEFAULT,
            image_shrink_standalone_to_fit: State::STATE_DEFAULT,
            text_area_resize: State::STATE_DEFAULT,
            tab_to_links: State::STATE_DEFAULT,
            local_storage: State::STATE_DEFAULT,
            databases: State::STATE_DEFAULT,
            webgl: State::STATE_DEFAULT,
            background_color: Default::default(),
            chrome_zoom_bubble: State::STATE_DEFAULT,
            chrome_status_bubble: State::STATE_DEFAULT,
        }
    }
}

impl BrowserSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_raw(self) -> cef_browser_settings_t {
        cef_browser_settings_t {
            size: std::mem::size_of::<cef_browser_settings_t>(),
            windowless_frame_rate: self.windowless_frame_rate as c_int,
            standard_font_family: self.standard_font_family.as_raw(),
            fixed_font_family: self.fixed_font_family.as_raw(),
            serif_font_family: self.serif_font_family.as_raw(),
            sans_serif_font_family: self.sans_serif_font_family.as_raw(),
            cursive_font_family: self.cursive_font_family.as_raw(),
            fantasy_font_family: self.fantasy_font_family.as_raw(),
            default_font_size: self.default_font_size as c_int,
            default_fixed_font_size: self.default_fixed_font_size as c_int,
            minimum_font_size: self.minimum_font_size as c_int,
            minimum_logical_font_size: self.minimum_logical_font_size as c_int,
            default_encoding: self.default_encoding.as_raw(),
            remote_fonts: self.remote_fonts,
            javascript: self.javascript,
            javascript_close_windows: self.javascript_close_windows,
            javascript_access_clipboard: self.javascript_access_clipboard,
            javascript_dom_paste: self.javascript_dom_paste,
            image_loading: self.image_loading,
            image_shrink_standalone_to_fit: self.image_shrink_standalone_to_fit,
            text_area_resize: self.text_area_resize,
            tab_to_links: self.tab_to_links,
            local_storage: self.local_storage,
            databases: self.databases,
            webgl: self.webgl,
            background_color: self.background_color,
            chrome_zoom_bubble: self.chrome_zoom_bubble,
            chrome_status_bubble: self.chrome_status_bubble,
        }
    }
}

/// See [cef_browser_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct Browser(cef_browser_t);

impl Browser {
    /// See [cef_browser_host_create_browser_sync] for more documentation.
    pub fn create<T: Client>(
        window_info: WindowInfo,
        client: Option<T>,
        url: CefString,
        settings: BrowserSettings,
    ) -> Result<Browser> {
        let client = client.map(|c| c.into_raw()).unwrap_or(null_mut());
        let ret = unsafe {
            cef_browser_host_create_browser_sync(
                &window_info.as_raw(),
                client,
                &url.as_raw(),
                &settings.as_raw(),
                null_mut(),
                null_mut(),
            )
        };
        if ret.is_null() {
            return Err(crate::error::Error::NullPtr);
        }

        Ok(unsafe { Self::from_raw(ret) })
    }

    /// See [cef_browser_view_get_for_browser]
    pub fn browrer_view(&self) -> BrowserView {
        unsafe { BrowserView::from_raw(cef_browser_view_get_for_browser(self.0.get_this())) }
    }
}
