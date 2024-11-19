use crate::wrapper;
use cef_sys::cef_jsdialog_handler_t;

wrapper! {
    #[doc = "See [cef_jsdialog_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct JsDialogHandler(cef_jsdialog_handler_t);
}

pub trait JsDialogCallback {}
