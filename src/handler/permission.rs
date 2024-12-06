use crate::prelude::*;

/// See [cef_permission_handler_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct PermissionHandler(cef_permission_handler_t);

pub trait PermissionCallback {}
