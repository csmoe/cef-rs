use crate::prelude::*;

#[doc = "See [cef_focus_handler_t] for more docs."]
#[derive(Debug, Clone)]
#[wrapper]
pub struct FocusHandler(cef_focus_handler_t);

pub trait FocusCallback {}
