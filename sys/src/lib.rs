#[allow(nonstandard_style, unused, clippy::all, rustfmt::skip)]
mod bindings;

pub use bindings::*;

impl Default for _cef_string_utf16_t {
    fn default() -> Self {
        Self {
            str_: std::ptr::null_mut(),
            length: 0,
            dtor: None,
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
