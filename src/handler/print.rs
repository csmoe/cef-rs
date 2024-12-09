use crate::prelude::*;

/// See [cef_print_handler_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct PrintHandler(cef_print_handler_t);
