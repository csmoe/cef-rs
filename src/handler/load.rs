use std::{ops::Deref, sync::Arc};

use crate::{prelude::*, CefBrowser, CefErrorCode, CefFrame, CefTransitionType};

/// See [cef_load_handler_t] for more docs.
#[wrapper]
pub struct LoadHandler(cef_load_handler_t);

/// See [cef_load_handler_t] for more docs.
pub trait CefLoadHandler {
    /// See [cef_load_handler_t::on_load_start]
    fn on_load_start(
        &self,
        browser: CefBrowser,
        frame: CefFrame,
        transition_type: CefTransitionType,
    ) {
    }

    /// See [cef_load_handler_t::on_load_end]
    fn on_load_end(&self, browser: CefBrowser, frame: CefFrame, httpStatusCode: u32) {}

    /// See [cef_load_handler_t::on_loading_state_change]
    fn on_loading_state_change(
        &self,
        browser: CefBrowser,
        isLoading: bool,
        canGoBack: bool,
        canGoForward: bool,
    ) {
    }

    /// See [cef_load_handler_t::on_load_error]
    fn on_load_error(
        &self,
        browser: CefBrowser,
        frame: CefFrame,
        errorCode: CefErrorCode,
        errorText: Option<CefString>,
        failedUrl: Option<CefString>,
    ) {
    }
}

pub struct CefLoadHandlerWrapper<T: CefLoadHandler> {
    inner: Arc<T>,
}

impl<T: CefLoadHandler> CefLoadHandlerWrapper<T> {
    pub fn new(load: T) -> Self {
        Self {
            inner: Arc::new(load),
        }
    }
}

impl<T: CefLoadHandler> Clone for CefLoadHandlerWrapper<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: CefLoadHandler> Deref for CefLoadHandlerWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: CefLoadHandler> CefLoadHandlerWrapper<T> {
    unsafe extern "C" fn on_loading_state_change(
        self_: *mut _cef_load_handler_t,
        browser: *mut _cef_browser_t,
        isLoading: ::std::os::raw::c_int,
        canGoBack: ::std::os::raw::c_int,
        canGoForward: ::std::os::raw::c_int,
    ) {
        let object: &crate::rc::RcImpl<_, Self> = crate::rc::RcImpl::get(self_);
        object.interface.on_loading_state_change(
            CefBrowser::from(browser),
            isLoading == 1,
            canGoBack == 1,
            canGoForward == 1,
        );
    }
    unsafe extern "C" fn on_load_start(
        self_: *mut _cef_load_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        transition_type: cef_transition_type_t,
    ) {
        let object: &crate::rc::RcImpl<_, Self> = crate::rc::RcImpl::get(self_);
        object.interface.on_load_start(
            CefBrowser::from(browser),
            CefFrame::from(frame),
            transition_type,
        );
    }

    unsafe extern "C" fn on_load_end(
        self_: *mut _cef_load_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        httpStatusCode: ::std::os::raw::c_int,
    ) {
        let object: &crate::rc::RcImpl<_, Self> = crate::rc::RcImpl::get(self_);
        object.interface.on_load_end(
            CefBrowser::from(browser),
            CefFrame::from(frame),
            httpStatusCode as _,
        );
    }
    #[allow(nonstandard_style)]
    unsafe extern "C" fn on_load_error(
        self_: *mut _cef_load_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        errorCode: cef_errorcode_t,
        errorText: *const cef_string_t,
        failedUrl: *const cef_string_t,
    ) {
        let object: &crate::rc::RcImpl<_, Self> = crate::rc::RcImpl::get(self_);
        object.interface.on_load_error(
            CefBrowser::from(browser),
            CefFrame::from(frame),
            errorCode,
            CefString::from_raw(errorText),
            CefString::from_raw(failedUrl),
        );
    }

    pub fn into_raw(self) -> *mut cef_load_handler_t {
        let mut object: cef_load_handler_t = unsafe { std::mem::zeroed() };
        object.on_load_end = Some(Self::on_load_end);
        object.on_load_start = Some(Self::on_load_start);
        object.on_load_error = Some(Self::on_load_error);
        object.on_loading_state_change = Some(Self::on_loading_state_change);

        crate::rc::RcImpl::new(object, self).cast()
    }
}
