use crate::prelude::*;
use core::ffi::c_void;

/// See [cef_dialog_handler_t] for more docs.
pub trait DialogHandler: Send + Sync {}
