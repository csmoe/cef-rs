use crate::prelude::*;

#[doc = "See [cef_keyboard_handler_t] for more docs."]
#[derive(Debug, Clone)]
#[wrapper]
pub struct KeyboardHandler(cef_keyboard_handler_t);

pub trait KeyboardCallback {}
