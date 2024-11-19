use crate::wrapper;
use cef_sys::cef_frame_t;

wrapper! {
    #[doc = "See [cef_frame_t] for more details."]
    #[derive(Debug, Clone)]
    pub struct Frame(cef_frame_t);
}
