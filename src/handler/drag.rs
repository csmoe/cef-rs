use crate::prelude::*;

#[doc = "See [cef_drag_handler_t] for more docs."]
#[derive(Debug, Clone)]
#[wrapper]
pub struct DragHandler(cef_drag_handler_t);

pub trait DragCallback {}
