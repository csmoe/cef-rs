use crate::handler::*;
use cef_sys::cef_client_t;

use crate::{rc::RcImpl, CefBrowser};

/// Handle browser-instance-specific callbacks
///
/// See [cef_client_t] for more documentation.
pub trait CefClient: Sized {
    type LifeSpan: CefLifeSpanHandler;
    type Render: CefRenderHandler;
    type ContextMenu: CefContextMenuHandler;
    type Load: CefLoadHandler;

    /// See [cef_client_t::get_life_span_handler]
    fn get_life_span_handler(&self) -> Option<CefLifeSpanWrapper<Self::LifeSpan>> {
        None
    }

    /// See [cef_client_t::get_render_handler]
    fn get_render_handler(&self) -> Option<Self::Render> {
        None
    }

    /// See [cef_client_t::get_request_handler]
    fn get_context_menu_handler(&self) -> Option<Self::ContextMenu> {
        None
    }
    // /// See [cef_client_t::get_audio_handler]
    // fn get_audio_handler(&self) -> Option<AudioHandler> {
    //     None
    // }

    // /// See [cef_client_t::get_command_handler]
    // fn get_command_handler(&self) -> Option<CommandHandler> {
    //     None
    // }

    // /// See [cef_client_t::get_request_handler]
    // fn get_dialog_handler<I: DialogHandler>(&self) -> Option<I> {
    //     None
    // }

    // /// See [cef_client_t::get_display_handler]
    // fn get_display_handler<I: DisplayHandler>(&self) -> Option<I> {
    //     None
    // }

    // /// See [cef_client_t::get_download_handler]
    // fn get_download_handler(&self) -> Option<DownloadHandler> {
    //     None
    // }

    // /// See [cef_client_t::get_drag_handler]
    // fn get_drag_handler(&self) -> Option<DragHandler> {
    //     None
    // }

    // /// See [cef_client_t::get_find_handler]
    // fn get_find_handler(&self) -> Option<FindHandler> {
    //     None
    // }

    // /// See [cef_client_t::get_focus_handler]
    // fn get_focus_handler(&self) -> Option<FocusHandler> {
    //     None
    // }

    // /// See [cef_client_t::get_frame_handler]
    // fn get_frame_handler(&self) -> Option<CefFrameHandler> {
    //     None
    // }

    // /// See [cef_client_t::get_permission_handler]
    // fn get_permission_handler(&self) -> Option<PermissionHandler> {
    //     None
    // }

    // /// See [cef_client_t::get_jsdialog_handler]
    // fn get_jsdialog_handler(&self) -> Option<JsDialogHandler> {
    //     None
    // }

    // /// See [cef_client_t::get_keyboard_handler]
    // fn get_keyboard_handler(&self) -> Option<KeyboardHandler> {
    //     None
    // }

    /// See [cef_client_t::get_load_handler]
    fn get_load_handler(&self) -> Option<CefLoadHandlerWrapper<Self::Load>> {
        None
    }

    // /// See [cef_client_t::get_print_handler]
    // fn get_print_handler(&self) -> Option<PrintHandler> {
    //     None
    // }

    // /// See [cef_client_t::get_request_handler]
    // fn get_request_handler(&self) -> Option<RequestHandler> {
    //     None
    // }

    /// See [cef_client_t::on_process_message_received]
    fn on_process_message_received(
        &self,
        _browser: CefBrowser,
        _frame: crate::CefFrame,
        _source_process: crate::CefProcessId,
        _message: crate::CefProcessMessage,
    ) -> bool {
        false
    }

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_client_t {
        let mut object: cef_client_t = unsafe { std::mem::zeroed() };
        //object.get_drag_handler = Some(get_drag_handler::<Self, H>);
        //object.get_find_handler = Some(get_find_handler::<Self, H>);
        object.get_load_handler = Some(get_load_handler::<Self>);
        //object.get_audio_handler = Some(get_audio_handler::<Self, H>);
        //object.get_focus_handler = Some(get_focus_handler::<Self, H>);
        //object.get_frame_handler = Some(get_frame_handler::<Self, H>);
        //object.get_print_handler = Some(get_print_handler::<Self, H>);
        //object.get_dialog_handler = Some(get_dialog_handler::<Self, H>);
        object.get_render_handler = Some(get_render_handler::<Self>);
        object.get_life_span_handler = Some(get_life_span_handler::<Self>);
        //object.get_command_handler = Some(get_command_handler::<Self, H>);
        //object.get_display_handler = Some(get_display_handler::<Self, H>);
        //object.get_request_handler = Some(get_request_handler::<Self, H>);
        //object.get_download_handler = Some(get_download_handler::<Self, H>);
        //object.get_permission_handler = Some(get_permission_handler::<Self, H>);
        object.get_context_menu_handler = Some(get_context_menu_handler::<Self>);
        object.on_process_message_received = Some(on_process_message_received::<Self>);

        RcImpl::new(object, self).cast()
    }
}

unsafe extern "C" fn get_render_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_render_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_render_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

unsafe extern "C" fn get_life_span_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_life_span_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface.get_life_span_handler().unwrap().into_raw()
    //.map(|h| h.into_raw())
    //.unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_context_menu_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_context_menu_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_context_menu_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn on_process_message_received<I: CefClient>(
    self_: *mut cef_sys::_cef_client_t,
    browser: *mut cef_sys::_cef_browser_t,
    frame: *mut cef_sys::_cef_frame_t,
    source_process: cef_sys::cef_process_id_t,
    message: *mut cef_sys::_cef_process_message_t,
) -> ::std::os::raw::c_int {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface.on_process_message_received(
        CefBrowser::from(browser),
        crate::CefFrame::from(frame),
        source_process,
        crate::CefProcessMessage::from(message),
    ) as _
}

pub(crate) unsafe extern "C" fn get_load_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_load_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_load_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

/*
pub(crate) unsafe extern "C" fn get_audio_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_audio_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_audio_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_request_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_request_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_request_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_keyboard_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_keyboard_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_keyboard_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_js_dialog_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_jsdialog_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_jsdialog_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_permission_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_permission_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_permission_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_frame_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_frame_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_frame_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_command_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_command_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_command_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}



pub(crate) unsafe extern "C" fn get_dialog_handler<I: CefClient, H: DialogHandler>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_dialog_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_dialog_handler::<H>()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_display_handler<I: CefClient, H: DisplayHandler>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_display_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_display_handler::<H>()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_download_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_download_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_download_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_drag_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_drag_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_drag_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_find_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_find_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_find_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_focus_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_focus_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_focus_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}



pub(crate) unsafe extern "C" fn get_print_handler<I: CefClient>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_print_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_print_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}
*/
