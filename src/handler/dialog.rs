use crate::prelude::*;

#[doc = "See [cef_dialog_handler_t] for more docs."]
#[derive(Debug, Clone)]
#[wrapper]
pub struct DialogHandler(cef_dialog_handler_t);

pub trait DialogCallback {}
