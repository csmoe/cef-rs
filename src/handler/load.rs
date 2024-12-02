use crate::prelude::*;

/// See [cef_load_handler_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct LoadHandler(cef_load_handler_t);

pub trait LoadCallback {}
