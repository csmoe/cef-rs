use cef_sys::cef_life_span_handler_t;

use crate::{string::CefString, Browser};

pub trait BrowserLifeSpanCallback {
    fn on_loading_state_change(
        &self,
        browser: Browser,
        is_loading: bool,
        can_go_back: bool,
        can_go_forward: bool,
    );
    fn on_load_start(
        &self,
        browser: Browser,
        frame: *mut cef_sys::cef_frame_t,
        transition_type: cef_sys::cef_transition_type_t,
    );
    fn on_load_end(
        &self,
        browser: Browser,
        frame: *mut cef_sys::cef_frame_t,
        http_status_code: i32,
    );
    fn on_load_error(
        &self,
        browser: Browser,
        frame: *mut cef_sys::cef_frame_t,
        error_code: cef_sys::cef_errorcode_t,
        error_text: CefString,
        failed_url: CefString,
    );
    fn do_close(&self, browser: Browser) -> bool;
    fn on_before_close(&self, browser: Browser);

    unsafe fn into_raw(&self) -> *mut cef_sys::cef_life_span_handler_t {
        let object: *mut cef_life_span_handler_t = unsafe { std::mem::zeroed() };
        crate::rc::RcImpl::new(object, self).cast()
    }
}

crate::wrapper! {
    #[doc = ""]
    #[derive(Clone)]
    pub struct BrowerLifeSpanHandler(cef_sys::cef_life_span_handler_t);
}
