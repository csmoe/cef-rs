#![doc = include_str!("../README.md")]

mod app;
pub mod args;
mod browser;
pub mod client;
mod command_line;
mod error;
mod handler;
mod image;
mod menu_model;
mod net;
mod prelude;
mod process_message;
pub mod rc;
mod settings;
mod string;
mod v8;
mod value;
mod view;

pub use app::*;
pub use browser::*;
pub use command_line::*;
pub use image::*;
pub use menu_model::*;
pub use process_message::ProcessMessage;
pub use settings::*;
pub use string::CefString;
pub use value::*;
pub use view::*;

mod alias {
    pub type LogSeverity = cef_sys::cef_log_severity_t;

    pub type LogItems = cef_sys::cef_log_items_t;

    /// The default value of `[Rect]` type is : { x: 0, y: 0, width: 1378, height: 800 }
    pub type Rect = cef_sys::cef_rect_t;

    pub type Size = cef_sys::cef_size_t;

    pub type State = cef_sys::cef_state_t;

    pub type GestureCommand = cef_sys::cef_gesture_command_t;

    pub type Point = cef_sys::cef_point_t;

    pub type Insets = cef_sys::cef_insets_t;

    pub type TextStyle = cef_sys::cef_text_style_t;

    pub type TextFieldCommands = cef_sys::cef_text_field_commands_t;

    pub type Range = cef_sys::cef_range_t;

    pub type AxisAlignment = cef_sys::cef_axis_alignment_t;

    pub type MenuColorType = cef_sys::cef_menu_color_type_t;

    pub type ProcessId = cef_sys::cef_process_id_t;

    pub type ValueType = cef_sys::cef_value_type_t;

    pub type RuntimeStyle = cef_sys::cef_runtime_style_t;

    pub type ChromeToolbarType = cef_sys::cef_chrome_toolbar_type_t;
}
pub use alias::*;
