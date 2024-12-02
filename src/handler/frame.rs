use crate::prelude::*;
use crate::{frame::Frame, Browser};

#[doc = "See [cef_frame_handler_t] for more docs."]
#[derive(Debug, Clone)]
#[wrapper]
pub struct FrameHandler(cef_frame_handler_t);

pub trait FrameCallback {
    fn on_frame_created(browser: Browser, frame: Frame);
    fn on_frame_attached(browser: Browser, frame: Frame, reattached: bool);
    fn on_frame_detached(browser: Browser, frame: Frame);
    fn on_main_frame_changed(browser: Browser, old_frame: Frame, new_frame: Frame);
}
