use crate::prelude::*;
use crate::{CefBrowser, CefFrame};

#[doc = "See [cef_frame_handler_t] for more docs."]
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefFrameHandler(cef_frame_handler_t);

pub trait CefFrameCallback {
    fn on_frame_created(browser: CefBrowser, frame: CefFrame);
    fn on_frame_attached(browser: CefBrowser, frame: CefFrame, reattached: bool);
    fn on_frame_detached(browser: CefBrowser, frame: CefFrame);
    fn on_main_frame_changed(browser: CefBrowser, old_frame: CefFrame, new_frame: CefFrame);
}
