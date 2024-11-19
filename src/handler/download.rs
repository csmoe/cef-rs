use crate::wrapper;
use cef_sys::cef_download_handler_t;

wrapper! {
    #[doc = "See [cef_download_handler_t] for more docs."]
    #[derive(Debug, Clone)]
    pub struct DownloadHandler(cef_download_handler_t);
}

pub trait DownloadCallback {}
