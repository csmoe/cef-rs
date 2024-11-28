use crate::wrapper;
use cef_sys::cef_context_menu_handler_t;

wrapper! {
    /// See [cef_context_menu_handler_t] for more docs.
    #[derive(Debug, Clone)]
    pub struct ContextMenuHandler(cef_context_menu_handler_t);
}

pub trait ContextMenuCallback {}
