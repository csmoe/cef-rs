use cef_sys::cef_jsdialog_handler_t;
use crate::wrapper;

wrapper! {
    #[doc = "See [cef_jsdialog_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct JsDialogHandler(cef_jsdialog_handler_t);
}

pub trait JsDialogCallback {}
