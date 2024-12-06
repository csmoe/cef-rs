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
    pub fn view(&self) -> View {
        unsafe { crate::view::View(self.0.convert()) }
    }

    /// See [cef_browser_view_create] for more documentation.
    pub fn create<T: Client>(
        client: Option<T>,
        url: &CefString,
        settings: BrowserSettings,
    ) -> Result<BrowserView> {
        let client = client.map(|c| c.into_raw()).unwrap_or(null_mut());

        let view = unsafe {
            cef_browser_view_create(
                client,
                &url.as_raw(),
                &settings.as_raw(),
                null_mut(),
                null_mut(),
                null_mut(),
            )
        };
        if view.is_null() {
            return Err(Error::CannotCreateBrowserView);
        }

        Ok(unsafe { BrowserView::from_raw(view) })
    }
}
