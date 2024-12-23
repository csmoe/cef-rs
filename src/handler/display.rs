use crate::prelude::*;

/// See [cef_display_handler_t] for more docs.
pub trait DisplayHandler: Sized + Default {
    /// See [cef_display_handler_t::on_address_change].
    fn on_address_change(&self, browser: crate::CefBrowser, frame: crate::CefFrame, url: CefString);

    /// See [cef_display_handler_t::on_title_change].
    fn on_title_change(&self, browser: crate::CefBrowser, title: CefString);

    /// See [cef_display_handler_t::on_favicon_urlchange].
    fn on_favicon_urlchange(&self, browser: crate::CefBrowser, icon_urls: cef_string_list_t);

    /// See [cef_display_handler_t::on_fullscreen_mode_change].
    fn on_fullscreen_mode_change(&self, browser: crate::CefBrowser, fullscreen: bool);

    /// See [cef_display_handler_t::on_tooltip].
    fn on_tooltip(&self, browser: crate::CefBrowser, text: CefString) -> bool;

    /// See [cef_display_handler_t::on_status_message].
    fn on_status_message(&self, browser: crate::CefBrowser, value: CefString);

    /// See [cef_display_handler_t::on_console_message].
    fn on_console_message(
        &self,
        browser: crate::CefBrowser,
        level: crate::CefLogSeverity,
        message: CefString,
        source: CefString,
        line: i32,
    ) -> bool;

    /// See [cef_display_handler_t::on_auto_resize].
    fn on_auto_resize(&self, browser: crate::CefBrowser, new_size: &crate::CefSize) -> bool;

    /// See [cef_display_handler_t::on_loading_progress_change].
    fn on_loading_progress_change(&self, browser: crate::CefBrowser, progress: f64);

    /// See [cef_display_handler_t::on_cursor_change].
    fn on_cursor_change(
        &self,
        browser: crate::CefBrowser,
        #[cfg(target_family = "unix")] cursor: *mut ::std::os::raw::c_void,
        #[cfg(target_os = "windows")] cursor: HCURSOR,
        type_: cef_cursor_type_t,
        custom_cursor_info: &cef_cursor_info_t,
    ) -> bool;

    /// See [cef_display_handler_t::on_media_access_change].
    fn on_media_access_change(
        &self,
        browser: crate::CefBrowser,
        has_video_access: bool,
        has_audio_access: bool,
    );

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_display_handler_t {
        let mut object: cef_display_handler_t = unsafe { std::mem::zeroed() };

        unsafe extern "C" fn on_address_change<I: DisplayHandler>(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            url: *const cef_string_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            let frame = crate::CefFrame::from(frame);
            let url = crate::CefString::from_raw(url).unwrap_or_default();
            object.interface.on_address_change(browser, frame, url);
        }

        unsafe extern "C" fn on_title_change<I: DisplayHandler>(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            title: *const cef_string_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            let title = crate::CefString::from_raw(title).unwrap_or_default();
            object.interface.on_title_change(browser, title);
        }

        unsafe extern "C" fn on_favicon_urlchange<I: DisplayHandler>(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            icon_urls: cef_string_list_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object.interface.on_favicon_urlchange(browser, icon_urls);
        }

        unsafe extern "C" fn on_fullscreen_mode_change<I: DisplayHandler>(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            fullscreen: ::std::os::raw::c_int,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object
                .interface
                .on_fullscreen_mode_change(browser, fullscreen != 0);
        }

        unsafe extern "C" fn on_tooltip<I: DisplayHandler>(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            text: *mut cef_string_t,
        ) -> ::std::os::raw::c_int {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            let text_str = crate::CefString::from_raw(text).unwrap_or_default();
            object.interface.on_tooltip(browser, text_str) as _
        }

        unsafe extern "C" fn on_status_message<I: DisplayHandler>(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            value: *const cef_string_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            let value = crate::CefString::from_raw(value).unwrap_or_default();
            object.interface.on_status_message(browser, value);
        }

        unsafe extern "C" fn on_console_message<I: DisplayHandler>(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            level: cef_log_severity_t,
            message: *const cef_string_t,
            source: *const cef_string_t,
            line: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            let message = crate::CefString::from_raw(message).unwrap_or_default();
            let source = crate::CefString::from_raw(source).unwrap_or_default();
            object
                .interface
                .on_console_message(browser, level, message, source, line)
                as ::std::os::raw::c_int
        }

        unsafe extern "C" fn on_auto_resize<I: DisplayHandler>(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            new_size: *const cef_size_t,
        ) -> ::std::os::raw::c_int {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object.interface.on_auto_resize(browser, &*new_size) as ::std::os::raw::c_int
        }

        unsafe extern "C" fn on_loading_progress_change<I: DisplayHandler>(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            progress: f64,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object
                .interface
                .on_loading_progress_change(browser, progress);
        }

        unsafe extern "C" fn on_cursor_change<I: DisplayHandler>(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            #[cfg(target_family = "unix")] cursor: *mut ::std::os::raw::c_void,
            #[cfg(target_os = "windows")] cursor: HCURSOR,
            type_: cef_cursor_type_t,
            custom_cursor_info: *const cef_cursor_info_t,
        ) -> ::std::os::raw::c_int {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object
                .interface
                .on_cursor_change(browser, cursor, type_, &*custom_cursor_info) as _
        }

        unsafe extern "C" fn on_media_access_change<I: DisplayHandler>(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            has_video_access: ::std::os::raw::c_int,
            has_audio_access: ::std::os::raw::c_int,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object.interface.on_media_access_change(
                browser,
                has_video_access != 0,
                has_audio_access != 0,
            );
        }

        object.on_address_change = Some(on_address_change::<Self>);
        object.on_title_change = Some(on_title_change::<Self>);
        object.on_favicon_urlchange = Some(on_favicon_urlchange::<Self>);
        object.on_fullscreen_mode_change = Some(on_fullscreen_mode_change::<Self>);
        object.on_tooltip = Some(on_tooltip::<Self>);
        object.on_status_message = Some(on_status_message::<Self>);
        object.on_console_message = Some(on_console_message::<Self>);
        object.on_auto_resize = Some(on_auto_resize::<Self>);
        object.on_loading_progress_change = Some(on_loading_progress_change::<Self>);
        object.on_cursor_change = Some(on_cursor_change::<Self>);
        object.on_media_access_change = Some(on_media_access_change::<Self>);

        crate::rc::RcImpl::new(object, self).cast()
    }
}
