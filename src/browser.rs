use crate::{
    client::CefClient, error::Result, prelude::*, string::CefString, view::CefWindowInfo,
    CefBrowserView, CefChromeToolbarType, CefState,
};
use cef_sys::{
    cef_browser_host_create_browser_sync, cef_browser_settings_t, cef_browser_t, cef_client_t,
    cef_gesture_command_t,
};
use std::{ffi::c_int, ptr::null_mut};

mod frame;
pub use frame::*;

/// See [cef_browser_settings_t] for more documentation.
#[derive(Debug, Clone)]
pub struct CefBrowserSettings {
    /// See [cef_browser_settings_t::windowless_frame_rate]
    pub windowless_frame_rate: usize,
    /// See [cef_browser_settings_t::standard_font_family]
    pub standard_font_family: Option<CefString>,
    /// See [cef_browser_settings_t::fixed_font_family]
    pub fixed_font_family: Option<CefString>,
    /// See [cef_browser_settings_t::serif_font_family]
    pub serif_font_family: Option<CefString>,
    /// See [cef_browser_settings_t::sans_serif_font_family]
    pub sans_serif_font_family: Option<CefString>,
    /// See [cef_browser_settings_t::cursive_font_family]
    pub cursive_font_family: Option<CefString>,
    /// See [cef_browser_settings_t::fantasy_font_family]
    pub fantasy_font_family: Option<CefString>,
    /// See [cef_browser_settings_t::default_font_size]
    pub default_font_size: u32,
    /// See [cef_browser_settings_t::default_fixed_font_size]
    pub default_fixed_font_size: u32,
    /// See [cef_browser_settings_t::minimum_font_size]
    pub minimum_font_size: u32,
    /// See [cef_browser_settings_t::minimum_logical_font_size]
    pub minimum_logical_font_size: u32,
    /// See [cef_browser_settings_t::default_encoding]
    pub default_encoding: Option<CefString>,
    /// See [cef_browser_settings_t::remote_fonts]
    pub remote_fonts: CefState,
    /// See [cef_browser_settings_t::javascript]
    pub javascript: CefState,
    /// See [cef_browser_settings_t::javascript_close_windows]
    pub javascript_close_windows: CefState,
    /// See [cef_browser_settings_t::javascript_access_clipboard]
    pub javascript_access_clipboard: CefState,
    /// See [cef_browser_settings_t::javascript_dom_paste]
    pub javascript_dom_paste: CefState,
    /// See [cef_browser_settings_t::image_loading]
    pub image_loading: CefState,
    /// See [cef_browser_settings_t::image_shrink_standalone_to_fit]
    pub image_shrink_standalone_to_fit: CefState,
    /// See [cef_browser_settings_t::text_area_resize]
    pub text_area_resize: CefState,
    /// See [cef_browser_settings_t::tab_to_links]
    pub tab_to_links: CefState,
    /// See [cef_browser_settings_t::local_storage]
    pub local_storage: CefState,
    /// See [cef_browser_settings_t::databases]
    pub databases: CefState,
    /// See [cef_browser_settings_t::webgl]
    pub webgl: CefState,
    /// See [cef_browser_settings_t::background_color]
    pub background_color: u32,
    /// See [cef_browser_settings_t::chrome_zoom_bubble]
    pub chrome_zoom_bubble: CefState,
    /// See [cef_browser_settings_t::chrome_status_bubble]
    pub chrome_status_bubble: CefState,
}

impl Default for CefBrowserSettings {
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
            remote_fonts: CefState::STATE_DEFAULT,
            javascript: CefState::STATE_DEFAULT,
            javascript_close_windows: CefState::STATE_DEFAULT,
            javascript_access_clipboard: CefState::STATE_DEFAULT,
            javascript_dom_paste: CefState::STATE_DEFAULT,
            image_loading: CefState::STATE_DEFAULT,
            image_shrink_standalone_to_fit: CefState::STATE_DEFAULT,
            text_area_resize: CefState::STATE_DEFAULT,
            tab_to_links: CefState::STATE_DEFAULT,
            local_storage: CefState::STATE_DEFAULT,
            databases: CefState::STATE_DEFAULT,
            webgl: CefState::STATE_DEFAULT,
            background_color: Default::default(),
            chrome_zoom_bubble: CefState::STATE_DEFAULT,
            chrome_status_bubble: CefState::STATE_DEFAULT,
        }
    }
}

impl CefBrowserSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_raw(self) -> cef_browser_settings_t {
        cef_browser_settings_t {
            size: std::mem::size_of::<cef_browser_settings_t>(),
            windowless_frame_rate: self.windowless_frame_rate as c_int,
            standard_font_family: self
                .standard_font_family
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            fixed_font_family: self
                .fixed_font_family
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            serif_font_family: self
                .serif_font_family
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            sans_serif_font_family: self
                .sans_serif_font_family
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            cursive_font_family: self
                .cursive_font_family
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            fantasy_font_family: self
                .fantasy_font_family
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            default_font_size: self.default_font_size as c_int,
            default_fixed_font_size: self.default_fixed_font_size as c_int,
            minimum_font_size: self.minimum_font_size as c_int,
            minimum_logical_font_size: self.minimum_logical_font_size as c_int,
            default_encoding: self
                .default_encoding
                .map(|v| v.as_raw())
                .unwrap_or_default(),
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

    pub fn from_raw(raw: cef_browser_settings_t) -> CefBrowserSettings {
        let cef_browser_settings_t {
            windowless_frame_rate,
            standard_font_family,
            fixed_font_family,
            serif_font_family,
            sans_serif_font_family,
            cursive_font_family,
            fantasy_font_family,
            default_font_size,
            default_fixed_font_size,
            minimum_font_size,
            minimum_logical_font_size,
            default_encoding,
            remote_fonts,
            javascript,
            javascript_close_windows,
            javascript_access_clipboard,
            javascript_dom_paste,
            image_loading,
            image_shrink_standalone_to_fit,
            text_area_resize,
            tab_to_links,
            local_storage,
            databases,
            webgl,
            background_color,
            chrome_zoom_bubble,
            chrome_status_bubble,
            ..
        } = raw;

        unsafe {
            CefBrowserSettings {
                windowless_frame_rate: windowless_frame_rate as usize,
                standard_font_family: CefString::from_raw(&standard_font_family),
                fixed_font_family: CefString::from_raw(&fixed_font_family),
                serif_font_family: CefString::from_raw(&serif_font_family),
                sans_serif_font_family: CefString::from_raw(&sans_serif_font_family),
                cursive_font_family: CefString::from_raw(&cursive_font_family),
                fantasy_font_family: CefString::from_raw(&fantasy_font_family),
                default_font_size: default_font_size as u32,
                default_fixed_font_size: default_fixed_font_size as u32,
                minimum_font_size: minimum_font_size as u32,
                minimum_logical_font_size: minimum_logical_font_size as u32,
                default_encoding: CefString::from_raw(&default_encoding),
                remote_fonts,
                javascript,
                javascript_close_windows,
                javascript_access_clipboard,
                javascript_dom_paste,
                image_loading,
                image_shrink_standalone_to_fit,
                text_area_resize,
                tab_to_links,
                local_storage,
                databases,
                webgl,
                background_color,
                chrome_zoom_bubble,
                chrome_status_bubble,
            }
        }
    }
}

/// See [cef_browser_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefBrowser(cef_browser_t);

impl CefBrowser {
    /// See [cef_browser_host_create_browser_sync] for more documentation.
    pub fn create<T: CefClient>(
        window_info: CefWindowInfo,
        client: Option<T>,
        url: CefString,
        settings: CefBrowserSettings,
    ) -> Result<CefBrowser> {
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
    pub fn browrer_view(&self) -> CefBrowserView {
        unsafe { CefBrowserView::from_raw(cef_browser_view_get_for_browser(self.0.get_this())) }
    }
}

#[derive(Debug, Clone)]
#[wrapper]
/// See [cef_browser_host_t] for more documentation.
pub struct CefBrowserHost(cef_browser_host_t);

impl CefBrowser {
    wrapper_methods! {
        /// See [cef_browser_t::is_valid]
        fn is_valid(&self) -> bool;

        /// See [cef_browser_t::get_host]
        fn get_host(&self) -> CefBrowserHost {
            self.0.get_host.map(|f| unsafe {
                CefBrowserHost::from_raw(f(self.0.get_this()))
            })
        }

        /// See [cef_browser_t::can_go_back]
        fn can_go_back(&self) -> bool ;

        /// See [cef_browser_t::go_back]
        fn go_back(&mut self) ;

        /// See [cef_browser_t::can_go_forward]
        fn can_go_forward(&self) -> bool ;

        /// See [cef_browser_t::go_forward]
        fn go_forward(&mut self);

        /// See [cef_browser_t::is_loading]
        fn is_loading(&self) -> bool ;
        /// See [cef_browser_t::reload]
        fn reload(&mut self);

        /// See [cef_browser_t::reload_ignore_cache]
        fn reload_ignore_cache(&self) ;

        /// See [cef_browser_t::stop_load]
        fn stop_load(&self) ;

        /// See [cef_browser_t::get_identifier]
        fn get_identifier(&self) -> i32 ;

        /// See [cef_browser_t::is_same]
        fn is_same(&self, that: CefBrowser) -> bool ;
        /// See [cef_browser_t::is_popup]
        fn is_popup(&self) -> bool ;
        /// See [cef_browser_t::has_document]
        fn has_document(&self) -> bool ;
        /// See [cef_browser_t::get_main_frame]
        fn get_main_frame(&self) -> CefFrame {
            self.0.get_main_frame.map(|f| unsafe {
                CefFrame::from_raw(f(self.0.get_this()))
            })
        }

        /// See [cef_browser_t::get_focused_frame]
        fn get_focused_frame(&self) -> CefFrame {
            self.0.get_focused_frame.map(|f| unsafe {
                CefFrame::from_raw(f(self.0.get_this()))
            })
        }

        /// See [cef_browser_t::get_frame_by_identifier]
        fn get_frame_by_identifier(&self, identifier: &str) -> CefFrame {
            self.0.get_frame_by_identifier.map(|f| unsafe {
                CefFrame::from_raw(f(self.0.get_this(),&CefString::from(identifier).as_raw()))
            })
        }

        /// See [cef_browser_t::get_frame_by_name]
        fn get_frame_by_name(&self, name: &str) -> CefFrame {
            self.0.get_frame_by_name.map(|f| unsafe {
                CefFrame::from_raw(f(self.0.get_this(), &CefString::from(name).as_raw()))
            })
        }

        /// See [cef_browser_t::get_frame_count]
        fn get_frame_count(&self) -> usize ;
        /// See [cef_browser_t::get_frame_identifiers]
        fn get_frame_identifiers(&self) -> Vec<CefString> {
            if let Some(f) = self.0.get_frame_identifiers {
                let list = std::ptr::null_mut();
                unsafe {
                    f(self.0.get_this(), list);
                    crate::string::parse_string_list(list).into()
                }
            } else { None }
        }

        /// See [cef_browser_t::get_frame_names]
        fn get_frame_names(&self) -> Vec<CefString> {
            if let Some(f) = self.0.get_frame_names {
                let list = std::ptr::null_mut();
                unsafe {
                    f(self.0.get_this(), list);
                    crate::string::parse_string_list(list).into()
                }
            } else { None }
        }
    }
}
