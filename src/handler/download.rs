use crate::prelude::*;

#[doc = "See [cef_download_handler_t] for more docs."]
#[derive(Debug, Clone)]
#[wrapper]
pub struct DownloadHandler(cef_download_handler_t);

pub trait DownloadCallback {}
