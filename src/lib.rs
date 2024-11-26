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
pub mod rc;
mod settings;
pub mod string;
mod view;

pub use app::*;
pub use browser::*;
pub use command_line::*;
pub use error::Error;
pub use error::Result;
pub use image::*;
pub use settings::*;
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
}
pub use alias::*;

macro_rules! wrapper {
    (
        $(#[$attr:meta])*
        pub struct $name:ident($sys:path);
        $(
            $visibility:vis fn $method:ident($(&$($mut:tt)?)? self $(, $arg:ident : $type:ty)* ) $(-> $ret:ty)?;
        )*
    ) => {
        $(#[$attr])*
        pub struct $name(pub(crate) $crate::rc::RefGuard<$sys>);

        impl $crate::rc::Rc for $sys {
            fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
                &self.base.as_base()
            }
        }

        impl $crate::rc::Rc for $name {
            fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
                self.0.as_base()
            }
        }

        impl From<*mut $sys> for $name {
            fn from(ptr: *mut $sys) -> Self {
                unsafe { $name($crate::rc::RefGuard::from_raw(ptr)) }
            }
        }

        impl From<$name> for *mut $sys {
            fn from(value: $name) -> Self {
                unsafe { value.into_raw() }
            }
        }

        impl From<$name> for *const $sys {
            fn from(value: $name) -> Self {
                unsafe { value.into_raw() }
            }
        }

        impl $name {
            #[allow(clippy::missing_safety_doc)]
            pub unsafe fn from_raw(ptr: *mut $sys) -> Self {
                Self($crate::rc::RefGuard::from_raw(ptr))
            }

            #[allow(clippy::missing_safety_doc)]
            pub unsafe fn into_raw(self) -> *mut $sys {
                self.0.into_raw()
            }
        }
    };
}

pub(crate) use wrapper;
