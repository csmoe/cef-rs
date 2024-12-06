use crate::prelude::*;

#[doc = "See [cef_find_handler_t] for more docs."]
#[derive(Debug, Clone)]
#[wrapper]
pub struct FindHandler(cef_find_handler_t);

pub trait FindCallback {}
