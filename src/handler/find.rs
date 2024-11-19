use cef_sys::cef_find_handler_t;
use crate::wrapper;

wrapper! {
    #[doc = "See [cef_find_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct FindHandler(cef_find_handler_t);
}

pub trait FindCallback {}
