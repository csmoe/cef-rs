use crate::prelude::*;

#[doc = "See [cef_display_handler_t] for more docs."]
#[derive(Debug, Clone)]
#[wrapper]
pub struct DisplayHandler(cef_display_handler_t);

pub trait DisplayCallback {}
