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
        fn can_go_back(&self) -> bool;

        /// See [cef_browser_t::go_back]
        fn go_back(&mut self);

        /// See [cef_browser_t::can_go_forward]
        fn can_go_forward(&self) -> bool;

        /// See [cef_browser_t::go_forward]
        fn go_forward(&mut self);

        /// See [cef_browser_t::is_loading]
        fn is_loading(&self) -> bool;
        /// See [cef_browser_t::reload]
        fn reload(&mut self);

        /// See [cef_browser_t::reload_ignore_cache]
        fn reload_ignore_cache(&self);

        /// See [cef_browser_t::stop_load]
        fn stop_load(&self);

        /// See [cef_browser_t::get_identifier]
        fn get_identifier(&self) -> i32;

        /// See [cef_browser_t::is_same]
        fn is_same(&self, that: CefBrowser) -> bool;
        /// See [cef_browser_t::is_popup]
        fn is_popup(&self) -> bool;
        /// See [cef_browser_t::has_document]
        fn has_document(&self) -> bool;
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
        fn get_frame_count(&self) -> usize;

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

#[derive(Debug, Clone)]
#[wrapper]
/// See [cef_browser_host_t] for more documentation.
pub struct CefBrowserHost(cef_browser_host_t);

/// FIXME
impl CefBrowserHost {
    wrapper_methods! {
                /// See [cef_browser_host_t::get_browser]
                fn get_browser(&self) ->CefBrowser {
                    self.0.get_browser.map(|f| unsafe {
                        CefBrowser::from_raw(f(self.0.get_this()))
                    })
                }

                /// See [cef_browser_host_t::close_browser]
                fn close_browser(&self, force_close: bool);

                /// See [cef_browser_host_t::try_close_browser]
                fn try_close_browser(&self) -> bool;

                /// See [cef_browser_host_t::is_ready_to_be_closed]
                fn is_ready_to_be_closed(&self) -> bool;

                /// See [cef_browser_host_t::set_focus]
                fn set_focus(&self, focus: bool);

                /// See [cef_browser_host_t::get_window_handle]
                #[cfg(target_os = "windows")]
                fn get_window_handle(&self) -> windows::Win32::Foundation::HWND {
                    let Some(f) = self.0.get_window_handle else { return None; };
                    unsafe { windows::Win32::Foundation::HWND(f(self.0.get_this()).cast()).into() }
                }

                /// See [cef_browser_host_t::get_opener_window_handle]
                #[cfg(target_os = "windows")]
                fn get_opener_window_handle(&self) -> windows::Win32::Foundation::HWND {
                    let Some(f) = self.0.get_opener_window_handle else { return None; };
                    unsafe { windows::Win32::Foundation::HWND(f(self.0.get_this()).cast()).into() }
                }

                /// See [cef_browser_host_t::has_view]
                fn has_view(&self) -> bool;

                /// See [cef_browser_host_t::get_client]
                ///fn get_client<C: CefClient>(&self) -> Option<C> {
                ///    self.0.get_client.map(|f| unsafe {
                ///        f(self.0.get_this())
                ///    })
                ///}

                /// See [cef_browser_host_t::get_request_context]
                //fn get_request_context(&self) -> Option<CefRequestContext> {
                //    self.0.get_request_context.map(|f| unsafe {
                //        CefRequestContext::from_raw(f(self.0.get_this()))
                //    })
                //}
    /*
                /// See [cef_browser_host_t::can_zoom]
                fn can_zoom(&self, command: cef_zoom_command_t) -> bool;

                /// See [cef_browser_host_t::zoom]
                fn zoom(&self, command: cef_zoom_command_t);

                /// See [cef_browser_host_t::get_default_zoom_level]
                fn get_default_zoom_level(&self) -> f64;

                /// See [cef_browser_host_t::get_zoom_level]
                fn get_zoom_level(&self) -> f64;

                /// See [cef_browser_host_t::set_zoom_level]
                fn set_zoom_level(&self, zoom_level: f64);

                /// See [cef_browser_host_t::run_file_dialog]
                //fn run_file_dialog(&self, mode: cef_file_dialog_mode_t, title: &str, default_file_path: &str, accept_filters: &[String], callback: CefRunFileDialogCallback);

                /// See [cef_browser_host_t::start_download]
                fn start_download(&self, url: &str);

                /// See [cef_browser_host_t::download_image]
                //fn download_image(&self, image_url: &str, is_favicon: bool, max_image_size: u32, bypass_cache: bool, callback: CefDownloadImageCallback);

                /// See [cef_browser_host_t::print]
                fn print(&self);

                /// See [cef_browser_host_t::print_to_pdf]
                ///fn print_to_pdf(&self, path: &str, settings: &CefPdfPrintSettings, callback: CefPdfPrintCallback);

                /// See [cef_browser_host_t::find]
                fn find(&self, search_text: &str, forward: bool, match_case: bool, find_next: bool);

                /// See [cef_browser_host_t::stop_finding]
                fn stop_finding(&self, clear_selection: bool);

                /// See [cef_browser_host_t::show_dev_tools]
                ///fn show_dev_tools(&self, window_info: &CefWindowInfo, client: CefClient, settings: &CefBrowserSettings, inspect_element_at: &CefPoint);

                /// See [cef_browser_host_t::close_dev_tools]
                fn close_dev_tools(&self);

                /// See [cef_browser_host_t::has_dev_tools]
                fn has_dev_tools(&self) -> bool;

                /// See [cef_browser_host_t::send_dev_tools_message]
                fn send_dev_tools_message(&self, message: &[u8]) -> bool;

                /// See [cef_browser_host_t::execute_dev_tools_method]
                ///fn execute_dev_tools_method(&self, message_id: i32, method: &str, params: Option<crate::CefDictionaryValue>) -> i32;

                /// See [cef_browser_host_t::add_dev_tools_message_observer]
                ///fn add_dev_tools_message_observer(&self, observer: CefDevToolsMessageObserver) -> CefRegistration;

                /// See [cef_browser_host_t::get_navigation_entries]
                ///fn get_navigation_entries(&self, visitor: CefNavigationEntryVisitor, current_only: bool);

                /// See [cef_browser_host_t::replace_misspelling]
                fn replace_misspelling(&self, word: &str);

                /// See [cef_browser_host_t::add_word_to_dictionary]
                fn add_word_to_dictionary(&self, word: &str);

                /// See [cef_browser_host_t::is_window_rendering_disabled]
                fn is_window_rendering_disabled(&self) -> bool;

                /// See [cef_browser_host_t::was_resized]
                fn was_resized(&self);

                /// See [cef_browser_host_t::was_hidden]
                fn was_hidden(&self, hidden: bool);

                /// See [cef_browser_host_t::notify_screen_info_changed]
                fn notify_screen_info_changed(&self);

                /// See [cef_browser_host_t::invalidate]
                fn invalidate(&self, element_type: cef_paint_element_type_t);

                /// See [cef_browser_host_t::send_external_begin_frame]
                fn send_external_begin_frame(&self);

                /// See [cef_browser_host_t::send_key_event]
                fn send_key_event(&self, event: &CefKeyEvent);

                /// See [cef_browser_host_t::send_mouse_click_event]
                fn send_mouse_click_event(&self, event: &CefMouseEvent, type_: cef_mouse_button_type_t, mouse_up: bool, click_count: i32);

                /// See [cef_browser_host_t::send_mouse_move_event]
                fn send_mouse_move_event(&self, event: &CefMouseEvent, mouse_leave: bool);

                /// See [cef_browser_host_t::send_mouse_wheel_event]
                fn send_mouse_wheel_event(&self, event: &CefMouseEvent, delta_x: i32, delta_y: i32);

                /// See [cef_browser_host_t::send_touch_event]
                fn send_touch_event(&self, event: &CefTouchEvent);

                /// See [cef_browser_host_t::send_capture_lost_event]
                fn send_capture_lost_event(&self);

                /// See [cef_browser_host_t::notify_move_or_resize_started]
                fn notify_move_or_resize_started(&self);

                /// See [cef_browser_host_t::get_windowless_frame_rate]
                fn get_windowless_frame_rate(&self) -> i32;

                /// See [cef_browser_host_t::set_windowless_frame_rate]
                fn set_windowless_frame_rate(&self, frame_rate: i32);

                /// See [cef_browser_host_t::ime_set_composition]
                fn ime_set_composition(&self, text: &str, underlines: &[CefCompositionUnderline], replacement_range: &CefRange, selection_range: &CefRange);

                /// See [cef_browser_host_t::ime_commit_text]
                fn ime_commit_text(&self, text: &str, replacement_range: &CefRange, relative_cursor_pos: i32);

                /// See [cef_browser_host_t::ime_finish_composing_text]
                fn ime_finish_composing_text(&self, keep_selection: bool);

                /// See [cef_browser_host_t::ime_cancel_composition]
                fn ime_cancel_composition(&self);

                /// See [cef_browser_host_t::drag_target_drag_enter]
                fn drag_target_drag_enter(&self, drag_data: CefDragData, event: &CefMouseEvent, allowed_ops: cef_drag_operations_mask_t);

                /// See [cef_browser_host_t::drag_target_drag_over]
                fn drag_target_drag_over(&self, event: &CefMouseEvent, allowed_ops: cef_drag_operations_mask_t);

                /// See [cef_browser_host_t::drag_target_drag_leave]
                fn drag_target_drag_leave(&self);

                /// See [cef_browser_host_t::drag_target_drop]
                fn drag_target_drop(&self, event: &CefMouseEvent);

                /// See [cef_browser_host_t::drag_source_ended_at]
                fn drag_source_ended_at(&self, x: i32, y: i32, op: cef_drag_operations_mask_t);

                /// See [cef_browser_host_t::drag_source_system_drag_ended]
                fn drag_source_system_drag_ended(&self);

                /// See [cef_browser_host_t::get_visible_navigation_entry]
                fn get_visible_navigation_entry(&self) -> Option<CefNavigationEntry> {
                    self.0.get_visible_navigation_entry.map(|f| unsafe {
                        CefNavigationEntry::from_raw(f(self.0.get_this()))
                    })
                }

                /// See [cef_browser_host_t::set_accessibility_state]
                fn set_accessibility_state(&self, state: cef_state_t);

                /// See [cef_browser_host_t::set_auto_resize_enabled]
                fn set_auto_resize_enabled(&self, enabled: bool, min_size: &CefSize, max_size: &CefSize);

                /// See [cef_browser_host_t::set_audio_muted]
                fn set_audio_muted(&self, mute: bool);

                /// See [cef_browser_host_t::is_audio_muted]
                fn is_audio_muted(&self) -> bool;

                /// See [cef_browser_host_t::is_fullscreen]
                fn is_fullscreen(&self) -> bool;

                /// See [cef_browser_host_t::exit_fullscreen]
                fn exit_fullscreen(&self, will_cause_resize: bool);

                /// See [cef_browser_host_t::can_execute_chrome_command]
                fn can_execute_chrome_command(&self, command_id: i32) -> bool;

                /// See [cef_browser_host_t::execute_chrome_command]
                fn execute_chrome_command(&self, command_id: i32, disposition: cef_window_open_disposition_t);

                /// See [cef_browser_host_t::is_render_process_unresponsive]
                fn is_render_process_unresponsive(&self) -> bool;
        */
                /// See [cef_browser_host_t::get_runtime_style]
                fn get_runtime_style(&self) -> cef_runtime_style_t;
            }
}
