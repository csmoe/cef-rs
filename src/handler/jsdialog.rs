use crate::prelude::*;

#[doc = "See [cef_jsdialog_handler_t] for more docs."]
#[derive(Debug, Clone)]
#[wrapper]
pub struct JsDialogHandler(cef_jsdialog_handler_t);

pub trait JsDialogCallback {}
