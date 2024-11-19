use crate::wrapper;
use cef_sys::cef_find_handler_t;

wrapper! {
    #[doc = "See [cef_find_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct FindHandler(cef_find_handler_t);
}

pub trait FindCallback {}
