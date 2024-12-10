use crate::{prelude::*, string::parse_string_list};

/// See [cef_dialog_handler_t] for more docs.
pub trait DialogHandler: Sized {
    /// See [cef_dialog_handler_t::on_file_dialog].
    fn on_file_dialog(
        &self,
        browser: crate::CefBrowser,
        mode: cef_file_dialog_mode_t,
        title: Option<CefString>,
        default_file_path: Option<CefString>,
        accept_filters: Vec<CefString>,
        accept_extensions: Vec<CefString>,
        accept_descriptions: Vec<CefString>,
        callback: cef_file_dialog_callback_t,
    ) -> bool;

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_dialog_handler_t {
        let mut object: cef_dialog_handler_t = unsafe { std::mem::zeroed() };

        unsafe extern "C" fn on_file_dialog<I: DialogHandler>(
            self_: *mut _cef_dialog_handler_t,
            browser: *mut _cef_browser_t,
            mode: cef_file_dialog_mode_t,
            title: *const cef_string_t,
            default_file_path: *const cef_string_t,
            accept_filters: cef_string_list_t,
            accept_extensions: cef_string_list_t,
            accept_descriptions: cef_string_list_t,
            callback: *mut _cef_file_dialog_callback_t,
        ) -> ::std::os::raw::c_int {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from_raw(browser);
            let title = if title.is_null() {
                None
            } else {
                Some(crate::CefString::from_raw(title).unwrap_or_default())
            };
            let default_file_path = if default_file_path.is_null() {
                None
            } else {
                Some(crate::CefString::from_raw(default_file_path).unwrap_or_default())
            };
            let accept_filters = parse_string_list(accept_filters);
            let accept_extensions = parse_string_list(accept_extensions);
            let accept_descriptions = parse_string_list(accept_descriptions);

            object.interface.on_file_dialog(
                browser,
                mode,
                title,
                default_file_path,
                accept_filters,
                accept_extensions,
                accept_descriptions,
                // FIXME
                *callback,
            ) as _
        }

        object.on_file_dialog = Some(on_file_dialog::<Self>);
        crate::rc::RcImpl::new(object, self).cast()
    }
}
