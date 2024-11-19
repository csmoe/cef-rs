use crate::wrapper;
use cef_sys::cef_permission_handler_t;

wrapper! {
    #[doc = "See [cef_permission_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct PermissionHandler(cef_permission_handler_t);
}

pub trait PermissionCallback {}
