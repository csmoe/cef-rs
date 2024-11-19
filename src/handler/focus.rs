use crate::wrapper;
use cef_sys::cef_focus_handler_t;

wrapper! {
    #[doc = "See [cef_focus_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct FocusHandler(cef_focus_handler_t);
}

pub trait FocusCallback {}
