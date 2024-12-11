use crate::{
    prelude::*, CefBrowserSettings, CefDictionaryValue, CefPopupFeatures, CefSettings,
    CefWindowInfo, CefWindowOpenDisposition,
};
use crate::{string::CefString, CefBrowser};

/// See [cef_life_span_handler_t] for more docs.
#[derive(Clone)]
#[wrapper]
pub struct BrowerLifeSpanHandler(cef_life_span_handler_t);

pub trait BrowserLifeSpanCallback {
    fn on_loading_state_change(
        &self,
        browser: CefBrowser,
        is_loading: bool,
        can_go_back: bool,
        can_go_forward: bool,
    );
    fn on_load_start(
        &self,
        browser: CefBrowser,
        frame: *mut cef_sys::cef_frame_t,
        transition_type: cef_sys::cef_transition_type_t,
    );
    fn on_load_end(
        &self,
        browser: CefBrowser,
        frame: *mut cef_sys::cef_frame_t,
        http_status_code: i32,
    );
    fn on_load_error(
        &self,
        browser: CefBrowser,
        frame: *mut cef_sys::cef_frame_t,
        error_code: cef_sys::cef_errorcode_t,
        error_text: CefString,
        failed_url: CefString,
    );
    fn do_close(&self, browser: CefBrowser) -> bool;
    fn on_before_close(&self, browser: CefBrowser);
}

/// See [cef_life_span_handler_t] for more docs.
#[allow(unused_variables)]
pub trait CefLifeSpanHandler: Sized {
    /// See [cef_life_span_handler_t::on_before_popup].
    fn on_before_popup(
        &self,
        browser: crate::CefBrowser,
        frame: crate::CefFrame,
        target_url: Option<CefString>,
        target_frame_name: Option<CefString>,
        target_disposition: CefWindowOpenDisposition,
        user_gesture: bool,
        popup_features: &CefPopupFeatures,
        window_info: Option<CefWindowInfo>,
        client: &mut *mut cef_client_t,
        settings: Option<CefBrowserSettings>,
        extra_info: Option<CefDictionaryValue>,
        no_javascript_access: &mut bool,
    ) -> bool {
        false
    }

    /// See [cef_life_span_handler_t::on_before_dev_tools_popup].
    fn on_before_dev_tools_popup(
        &self,
        browser: crate::CefBrowser,
        window_info: &mut _cef_window_info_t,
        client: &mut *mut _cef_client_t,
        settings: &mut _cef_browser_settings_t,
        extra_info: &mut *mut _cef_dictionary_value_t,
        use_default_window: &mut bool,
    ) {
    }

    /// See [cef_life_span_handler_t::on_after_created].
    fn on_after_created(&self, browser: crate::CefBrowser) {}

    /// See [cef_life_span_handler_t::do_close].
    fn do_close(&self, browser: crate::CefBrowser) -> bool {
        false
    }

    /// See [cef_life_span_handler_t::on_before_close].
    fn on_before_close(&self, browser: crate::CefBrowser) {}

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_life_span_handler_t {
        let mut object: cef_life_span_handler_t = unsafe { std::mem::zeroed() };

        unsafe extern "C" fn on_before_popup<I: CefLifeSpanHandler>(
            self_: *mut _cef_life_span_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            target_url: *const cef_string_t,
            target_frame_name: *const cef_string_t,
            target_disposition: cef_window_open_disposition_t,
            user_gesture: ::std::os::raw::c_int,
            popup_features: *const cef_popup_features_t,
            window_info: *mut _cef_window_info_t,
            client: *mut *mut _cef_client_t,
            settings: *mut _cef_browser_settings_t,
            extra_info: *mut *mut _cef_dictionary_value_t,
            no_javascript_access: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from_raw(browser);
            let frame = crate::CefFrame::from_raw(frame);
            let target_url = crate::CefString::from_raw(target_url);
            let target_frame_name = crate::CefString::from_raw(target_frame_name);
            let user_gesture = user_gesture != 0;
            let popup_features = &*popup_features;
            let window_info = CefWindowInfo::from_raw(window_info);
            let client = &mut *client;
            let settings = if settings.is_null() {
                None
            } else {
                CefBrowserSettings::from_raw(*settings).into()
            };
            let extra_info = if extra_info.is_null() {
                None
            } else {
                CefDictionaryValue::from_raw(*extra_info).into()
            };
            let mut no_js_access = *no_javascript_access != 0;

            let result = object.interface.on_before_popup(
                browser,
                frame,
                target_url,
                target_frame_name,
                target_disposition,
                user_gesture,
                popup_features,
                window_info,
                client,
                settings,
                extra_info,
                &mut no_js_access,
            );

            *no_javascript_access = if no_js_access { 1 } else { 0 };

            if result {
                1
            } else {
                0
            }
        }

        unsafe extern "C" fn on_before_dev_tools_popup<I: CefLifeSpanHandler>(
            self_: *mut _cef_life_span_handler_t,
            browser: *mut _cef_browser_t,
            window_info: *mut _cef_window_info_t,
            client: *mut *mut _cef_client_t,
            settings: *mut _cef_browser_settings_t,
            extra_info: *mut *mut _cef_dictionary_value_t,
            use_default_window: *mut ::std::os::raw::c_int,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from_raw(browser);
            let window_info = &mut *window_info;
            let client = &mut *client;
            let settings = &mut *settings;
            let extra_info = &mut *extra_info;
            let mut use_default = *use_default_window != 0;

            object.interface.on_before_dev_tools_popup(
                browser,
                window_info,
                client,
                settings,
                extra_info,
                &mut use_default,
            );

            *use_default_window = if use_default { 1 } else { 0 };
        }

        unsafe extern "C" fn on_after_created<I: CefLifeSpanHandler>(
            self_: *mut _cef_life_span_handler_t,
            browser: *mut _cef_browser_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from_raw(browser);
            object.interface.on_after_created(browser);
        }

        unsafe extern "C" fn do_close<I: CefLifeSpanHandler>(
            self_: *mut _cef_life_span_handler_t,
            browser: *mut _cef_browser_t,
        ) -> ::std::os::raw::c_int {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from_raw(browser);
            let result = object.interface.do_close(browser);
            if result {
                1
            } else {
                0
            }
        }

        unsafe extern "C" fn on_before_close<I: CefLifeSpanHandler>(
            self_: *mut _cef_life_span_handler_t,
            browser: *mut _cef_browser_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from_raw(browser);
            object.interface.on_before_close(browser);
        }

        object.on_before_popup = Some(on_before_popup::<Self>);
        object.on_before_dev_tools_popup = Some(on_before_dev_tools_popup::<Self>);
        object.on_after_created = Some(on_after_created::<Self>);
        object.do_close = Some(do_close::<Self>);
        object.on_before_close = Some(on_before_close::<Self>);
        crate::rc::RcImpl::new(object, self).cast()
    }
}

impl CefLifeSpanHandler for () {
    fn into_raw(self) -> *mut cef_life_span_handler_t {
        std::ptr::null_mut()
    }
}
