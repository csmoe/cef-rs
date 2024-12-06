use crate::prelude::*;

/// See [cef_context_menu_handler_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct ContextMenuHandler(cef_context_menu_handler_t);

pub trait ContextMenuCallback {}
