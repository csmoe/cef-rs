#![doc = include_str!("../README.md")]

mod interface;

mod app;
mod args;
mod browser;
mod client;
mod command_line;
mod error;
mod handler;
mod image;
mod menu_model;
mod multimap;
mod net;
mod preference_manager;
mod prelude;
mod process_message;
mod rc;
mod scoped;
mod settings;
mod string;
mod v8;
mod value;
mod view;

pub use app::*;
pub use args::*;
pub use browser::*;
pub use cef_sys as sys;
pub use client::*;
pub use command_line::*;
pub use error::*;
pub use handler::*;
pub use image::*;
pub use menu_model::*;
pub use net::*;
pub use process_message::CefProcessMessage;
pub use settings::*;
pub use string::CefString;
pub use v8::*;
pub use value::*;
pub use view::*;

mod alias {
    pub type CefLogSeverity = cef_sys::cef_log_severity_t;

    pub type CefLogItems = cef_sys::cef_log_items_t;

    /// The default value of `[Rect]` type is : { x: 0, y: 0, width: 1378, height: 800 }
    pub type CefRect = cef_sys::cef_rect_t;

    pub type CefSize = cef_sys::cef_size_t;

    pub type CefState = cef_sys::cef_state_t;

    pub type CefGestureCommand = cef_sys::cef_gesture_command_t;

    pub type CefPoint = cef_sys::cef_point_t;

    pub type CefInsets = cef_sys::cef_insets_t;

    pub type CefTextStyle = cef_sys::cef_text_style_t;

    pub type CefTextFieldCommands = cef_sys::cef_text_field_commands_t;

    pub type CefRange = cef_sys::cef_range_t;

    pub type CefAxisAlignment = cef_sys::cef_axis_alignment_t;

    pub type CefMenuColorType = cef_sys::cef_menu_color_type_t;

    pub type CefProcessId = cef_sys::cef_process_id_t;

    pub type CefValueType = cef_sys::cef_value_type_t;

    pub type CefRuntimeStyle = cef_sys::cef_runtime_style_t;

    pub type CefChromeToolbarType = cef_sys::cef_chrome_toolbar_type_t;

    pub type CefBaseTime = cef_sys::cef_basetime_t;

    pub type CefV8PropertyAttribute = cef_sys::cef_v8_propertyattribute_t;

    pub type CefPostDataElementType = cef_sys::cef_postdataelement_type_t;

    pub type CefPreferencesType = cef_sys::cef_preferences_type_t;

    pub type CefPopupFeatures = cef_sys::cef_popup_features_t;

    pub type CefWindowOpenDisposition = cef_sys::cef_window_open_disposition_t;

    pub type CefCookieSameSite = cef_sys::cef_cookie_same_site_t;

    pub type CefCookiePriority = cef_sys::cef_cookie_priority_t;
}
pub use alias::*;
