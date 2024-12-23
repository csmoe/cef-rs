use crate::prelude::*;
use crate::{
    client::CefClient,
    error::{Error, Result},
    string::CefString,
    view::CefView,
    CefBrowserSettings,
};
use cef_sys::{cef_browser_view_create, cef_browser_view_t};
use std::ptr::null_mut;

/// See [cef_browser_view_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefBrowserView(cef_browser_view_t);

impl CefBrowserView {
    /// See [cef_browser_view_create] for more documentation.
    pub fn create<T: CefClient>(
        client: Option<T>,
        url: &CefString,
        settings: CefBrowserSettings,
    ) -> Result<CefBrowserView> {
        let client = client.map(|c| c.into_raw()).unwrap_or(null_mut());
        let request_context = crate::net::CefRequestContext::global();

        let view = unsafe {
            cef_browser_view_create(
                client,
                &url.as_raw(),
                &settings.as_raw(),
                null_mut(),
                request_context.into_raw(),
                null_mut(),
            )
        };
        if view.is_null() {
            return Err(Error::CannotCreateBrowserView);
        }

        Ok(CefBrowserView::from(view))
    }

    wrapper_methods! {
        /// See [cef_browser_view_t::get_browser] for more documentation.
        fn get_browser(&self) -> crate::browser::CefBrowser {
             get_browser.and_then(|f| unsafe {
                let browser = f(self.get_this());
                if browser.is_null() {
                    None
                } else {
                    Some(crate::browser::CefBrowser::from(browser))
                }
            })
        }
    }
}

pub trait BrowserViewDelegate: Sized {
    /// See [cef_sys::cef_browser_view_delegate_t::on_browser_created]
    fn on_browser_created(&self, _browser_view: CefBrowserView, _browser: crate::CefBrowser) {}

    /// See [cef_sys::cef_browser_view_delegate_t::on_browser_destroyed]
    fn on_browser_destroyed(&self, _browser_view: CefBrowserView, _browser: crate::CefBrowser) {}

    /// See [cef_sys::cef_browser_view_delegate_t::on_gesture_command]
    fn on_gesture_command(
        &self,
        _browser_view: CefBrowserView,
        _gesture_command: cef_gesture_command_t,
    ) -> bool {
        todo!()
    }

    /// See [cef_sys::cef_browser_view_delegate_t::get_delegate_for_popup_browser_view]
    fn get_delegate_for_popup_browser_view(
        &self,
        _browser_view: CefBrowserView,
        _settings: CefBrowserSettings,
        _client: cef_client_t,
        _is_devtools: bool,
    ) -> Option<Self> {
        None
    }

    /// See [cef_sys::cef_browser_view_delegate_t::on_popup_browser_view_created]
    fn on_popup_browser_view_created(
        &self,
        _browser_view: CefBrowserView,
        _popup_browser_view: CefBrowserView,
        _is_devtools: bool,
    ) {
    }

    /// See [cef_sys::cef_browser_view_delegate_t::get_chrome_toolbar_type]
    fn get_chrome_toolbar_type(
        &self,
        _browser_view: CefBrowserView,
    ) -> crate::CefChromeToolbarType {
        todo!()
    }

    /// See [cef_sys::cef_browser_view_delegate_t::use_frameless_window_for_picture_in_picture]
    fn use_frameless_window_for_picture_in_picture(&self, _browser_view: CefBrowserView) -> bool {
        todo!()
    }

    /// See [cef_sys::cef_browser_view_delegate_t::get_browser_runtime_style]
    fn get_browser_runtime_style(&self) -> crate::CefRuntimeStyle {
        todo!()
    }
}
