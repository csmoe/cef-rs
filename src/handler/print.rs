
use cef_sys::cef_print_handler_t;
use crate::wrapper;

wrapper! {
    #[doc = "See [cef_print_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct PrintHandler(cef_print_handler_t);
}

pub trait PrintCallback {}
