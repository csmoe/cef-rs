use crate::prelude::*;
use crate::{
    client::Client,
    error::{Error, Result},
    string::CefString,
    view::View,
    BrowserSettings,
};
use cef_sys::{cef_browser_view_create, cef_browser_view_t};
use std::ptr::null_mut;

/// See [cef_browser_view_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct BrowserView(cef_browser_view_t);

impl BrowserView {
    /// See [cef_browser_view_create] for more documentation.
    pub fn create<T: Client>(
        client: Option<T>,
        url: &CefString,
        settings: BrowserSettings,
    ) -> Result<BrowserView> {
        let client = client.map(|c| c.into_raw()).unwrap_or(null_mut());
        let request_context = crate::net::RequestContext::global();

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

        Ok(unsafe { BrowserView::from_raw(view) })
    }

    pub fn view(&self) -> View {
        unsafe { crate::view::View(self.0.convert()) }
    }

    wrapper_methods! {
        /// See [cef_browser_view_t::get_browser] for more documentation.
        fn get_browser(&self) -> crate::browser::Browser {
            self.0.get_browser.and_then(|f| unsafe {
                let browser = f(self.0.get_this());
                if browser.is_null() {
                    None
                } else {
                    Some(crate::browser::Browser::from_raw(browser))
                }
            })
        }
    }
}

pub trait BrowserViewDelegate: Sized {
    /// See [cef_sys::cef_browser_view_delegate_t::on_browser_created]
    fn on_browser_created(&self, _browser_view: BrowserView, _browser: crate::Browser) {}

    /// See [cef_sys::cef_browser_view_delegate_t::on_browser_destroyed]
    fn on_browser_destroyed(&self, _browser_view: BrowserView, _browser: crate::Browser) {}

    /// See [cef_sys::cef_browser_view_delegate_t::on_gesture_command]
    fn on_gesture_command(
        &self,
        _browser_view: BrowserView,
        _gesture_command: cef_gesture_command_t,
    ) -> bool {
        todo!()
    }

    /// See [cef_sys::cef_browser_view_delegate_t::get_delegate_for_popup_browser_view]
    fn get_delegate_for_popup_browser_view(
        &self,
        _browser_view: BrowserView,
        _settings: BrowserSettings,
        _client: cef_client_t,
        _is_devtools: bool,
    ) -> Option<Self> {
        None
    }

    /// See [cef_sys::cef_browser_view_delegate_t::on_popup_browser_view_created]
    fn on_popup_browser_view_created(
        &self,
        _browser_view: BrowserView,
        _popup_browser_view: BrowserView,
        _is_devtools: bool,
    ) {
    }

    /// See [cef_sys::cef_browser_view_delegate_t::get_chrome_toolbar_type]
    fn get_chrome_toolbar_type(&self, _browser_view: BrowserView) -> crate::ChromeToolbarType {
        todo!()
    }

    /// See [cef_sys::cef_browser_view_delegate_t::use_frameless_window_for_picture_in_picture]
    fn use_frameless_window_for_picture_in_picture(&self, _browser_view: BrowserView) -> bool {
        todo!()
    }

    /// See [cef_sys::cef_browser_view_delegate_t::get_browser_runtime_style]
    fn get_browser_runtime_style(&self) -> crate::RuntimeStyle {
        todo!()
    }
}
