use crate::wrapper;
use cef_sys::cef_load_handler_t;

wrapper! {
    #[doc = "See [cef_load_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct LoadHandler(cef_load_handler_t);
}

pub trait LoadCallback {}
