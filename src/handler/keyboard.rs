use cef_sys::cef_keyboard_handler_t;
use crate::wrapper;

wrapper! {
    #[doc = "See [cef_keyboard_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct KeyboardHandler(cef_keyboard_handler_t);
}

pub trait KeyboardCallback {}
