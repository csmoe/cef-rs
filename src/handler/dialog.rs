use crate::wrapper;
use cef_sys::cef_dialog_handler_t;

wrapper! {
    #[doc = "See [cef_dialog_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct DialogHandler(cef_dialog_handler_t);
}

pub trait DialogCallback {}
