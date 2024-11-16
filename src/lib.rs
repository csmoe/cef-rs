#![doc = include_str!("../README.md")]

mod app;
pub mod args;
mod browser;
pub mod client;
mod command_line;
mod error;
pub mod rc;
mod settings;
pub mod string;
mod view;

pub use cef_sys as sys;

pub use app::*;
pub use browser::*;
pub use command_line::*;
pub use settings::*;
pub use view::*;

pub type LogSeverity = cef_sys::cef_log_severity_t;
pub type LogItems = cef_sys::cef_log_items_t;

/// The default value of `[Rect]` type is : { x: 0, y: 0, width: 1378, height: 800 }
pub type Rect = cef_sys::cef_rect_t;

pub type State = cef_sys::cef_state_t;
