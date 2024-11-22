#[allow(nonstandard_style, unused, clippy::all, rustfmt::skip)]
mod bindings;

pub use bindings::*;

pub use cef_wrapper_macro::FfiRc;

/// A marker trait for cef types that are reference counted(contains a [cef_base_ref_counted_t] or nested [cef_base_ref_counted_t]).
pub trait FfiRc {}

impl Default for cef_string_utf16_t {
    fn default() -> Self {
        Self {
            str_: std::ptr::null_mut(),
            length: 0,
            dtor: None,
        }
    }
}

impl Default for cef_rect_t {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 1378,
            height: 800,
        }
    }
}

impl Default for cef_size_t {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}

macro_rules! impl_default_for_enums {
    ($($type:ty => $default:ident),+ $(,)?) => {
        $(
            impl Default for $type {
                fn default() -> Self {
                    Self::$default
                }
            }
        )+
    };
}

impl_default_for_enums! {
    cef_log_severity_t => LOGSEVERITY_DEFAULT,
    cef_log_items_t => LOG_ITEMS_DEFAULT,
    cef_state_t => STATE_DEFAULT,
    cef_runtime_style_t => CEF_RUNTIME_STYLE_DEFAULT
}
