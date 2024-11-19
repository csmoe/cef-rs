
use cef_sys::cef_request_handler_t;
use crate::wrapper;

wrapper! {
    #[doc = "See [cef_request_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct RequestHandler(cef_request_handler_t);
}

pub trait RequestCallback {}
