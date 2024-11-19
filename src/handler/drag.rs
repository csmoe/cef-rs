use crate::wrapper;
use cef_sys::cef_drag_handler_t;

wrapper! {
    #[doc = "See [cef_drag_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct DragHandler(cef_drag_handler_t);
}

pub trait DragCallback {}
