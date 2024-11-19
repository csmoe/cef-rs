use crate::wrapper;
use cef_sys::cef_display_handler_t;

wrapper! {
    #[doc = "See [cef_display_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct DisplayHandler(cef_display_handler_t);
}

pub trait DisplayCallback {}
