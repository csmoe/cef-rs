use crate::prelude::*;

/// See [cef_request_handler_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct RequestHandler(cef_request_handler_t);

pub trait RequestCallback {}
