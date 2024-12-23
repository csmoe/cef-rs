use crate::{handler, prelude::*, CefBrowser, CefFrame};

/// See [cef_request_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefRequest(cef_request_t);

impl CefRequest {
    pub fn create() -> Result<CefRequest> {
        let ptr = unsafe { cef_request_create() };
        if ptr.is_null() {
            Err(crate::error::Error::NullPtr)
        } else {
            Ok(CefRequest::from(ptr))
        }
    }
}

impl CefRequest {
    wrapper_methods!(
        /// See [cef_request_t::is_read_only]
        fn is_read_only(&self) -> bool;

        /// See [cef_request_t::get_url]
        fn get_url(&self) -> CefString {
            get_url.and_then(|f| unsafe { CefString::from_userfree_cef(f(self.get_this())) })
        }

        /// See [cef_request_t::set_url]
        fn set_url(&self, url: &str) {
            set_url.map(|f| unsafe { f(self.get_this(), &CefString::from(url).as_raw()) })
        }

        /// See [cef_request_t::get_method]
        fn get_method(&self) -> CefString {
            get_method.and_then(|f| unsafe { CefString::from_userfree_cef(f(self.get_this())) })
        }

        /// See [cef_request_t::set_method]
        fn set_method(&self, method: &str) {
            set_method.map(|f| unsafe { f(self.get_this(), &CefString::from(method).as_raw()) })
        }

        /// See [cef_request_t::set_referrer]
        fn set_referrer(&self, referrer_url: &str, policy: cef_referrer_policy_t) {
            set_referrer.map(|f| unsafe {
                f(
                    self.get_this(),
                    &CefString::from(referrer_url).as_raw(),
                    policy as _,
                )
            })
        }

        /// See [cef_request_t::get_referrer_url]
        fn get_referrer_url(&self) -> CefString {
            get_referrer_url
                .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.get_this())) })
        }

        /// See [cef_request_t::get_referrer_policy]
        fn get_referrer_policy(&self) -> cef_referrer_policy_t;

        /// See [cef_request_t::get_post_data]
        fn get_post_data(&self) -> crate::net::CefPostData {
            get_post_data.and_then(|f| unsafe {
                let v = f(self.get_this());
                if v.is_null() {
                    return None;
                }
                crate::net::CefPostData::from(v).into()
            })
        }

        /// See [cef_request_t::set_post_data]
        fn set_post_data(&self, post_data: crate::net::CefPostData);

        /// See [cef_request_t::get_header_map]
        fn get_header_map(&self) -> crate::multimap::CefStringMultiMap {
            get_header_map.map(|f| unsafe {
                let map = std::ptr::null_mut();
                f(self.get_this(), map);
                crate::multimap::CefStringMultiMap::from_raw(map)
            })
        }

        /// See [cef_request_t::set_header_map]
        fn set_header_map(&self, header_map: crate::multimap::CefStringMultiMap) {
            set_header_map.map(|f| unsafe { f(self.get_this(), header_map.as_raw()) })
        }

        /// See [cef_request_t::get_header_by_name]
        fn get_header_by_name(&self, name: &str) -> CefString {
            get_header_by_name.and_then(|f| unsafe {
                CefString::from_userfree_cef(f(self.get_this(), &CefString::from(name).as_raw()))
            })
        }

        /// See [cef_request_t::set_header_by_name]
        fn set_header_by_name(&self, name: &str, value: &str, overwrite: bool) {
            set_header_by_name.map(|f| unsafe {
                f(
                    self.get_this(),
                    &CefString::from(name).as_raw(),
                    &CefString::from(value).as_raw(),
                    overwrite as _,
                )
            })
        }

        /// See [cef_request_t::set]
        fn set(
            &self,
            url: &str,
            method: &str,
            post_data: crate::net::CefPostData,
            header_map: crate::multimap::CefStringMultiMap,
        ) {
            set.map(|f| unsafe {
                f(
                    self.get_this(),
                    &CefString::from(url).as_raw(),
                    &CefString::from(method).as_raw(),
                    post_data.into_raw(),
                    header_map.as_raw(),
                )
            })
        }

        /// See [cef_request_t::get_flags]
        fn get_flags(&self) -> i32;

        /// See [cef_request_t::set_flags]
        fn set_flags(&self, flags: i32);

        /// See [cef_request_t::get_first_party_for_cookies]
        fn get_first_party_for_cookies(&self) -> CefString {
            get_first_party_for_cookies
                .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.get_this())) })
        }

        /// See [cef_request_t::set_first_party_for_cookies]
        fn set_first_party_for_cookies(&self, url: &str) {
            set_first_party_for_cookies
                .map(|f| unsafe { f(self.get_this(), &CefString::from(url).as_raw()) })
        }

        /// See [cef_request_t::get_resource_type]
        fn get_resource_type(&self) -> cef_resource_type_t;

        /// See [cef_request_t::get_transition_type]
        fn get_transition_type(&self) -> cef_transition_type_t;

        /// See [cef_request_t::get_identifier]
        fn get_identifier(&self) -> u64;
    );
}

/// See [cef_request_context_t] for more docs.
#[wrapper]
#[derive(Debug, Clone)]
pub struct CefRequestContext(cef_request_context_t);

impl CefRequestContext {
    /// See [cef_request_context_get_global_context]
    pub fn global() -> Self {
        unsafe { CefRequestContext::from(cef_request_context_get_global_context()) }
    }

    /// See [cef_request_context_create_context]
    pub fn create(
        settings: CefRequestContextSettings,
        handler: impl CefRequestContextHandler,
    ) -> Self {
        unsafe {
            CefRequestContext::from(cef_request_context_create_context(
                &settings.into_raw(),
                handler.into_raw(),
            ))
        }
    }

    /// See [cef_create_context_shared]
    pub fn create_shared(other: Self, handler: impl CefRequestContextHandler) -> Self {
        unsafe {
            CefRequestContext::from(cef_create_context_shared(
                other.into_raw(),
                handler.into_raw(),
            ))
        }
    }

    wrapper_methods! {
        /// See [cef_request_context_t::get_cache_path]
        fn get_cache_path(&self) -> CefString {
            unsafe {get_cache_path.and_then(|f| CefString::from_raw(f(self.get_this())))}
        }

        /// See [cef_request_context_t::get_handler]
        fn get_handler(&self) -> *mut cef_request_context_handler_t {
            unsafe {
                get_handler.map(|f| f(self.get_this()))
            }
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct CefRequestContextSettings {
    pub cache_path: CefString,
    pub persist_session_cookies: bool,
    pub accept_language_list: CefString,
    pub cookieable_schemes_list: CefString,
    pub cookieable_schemes_exclude_defaults: bool,
}

impl CefRequestContextSettings {
    pub fn into_raw(&self) -> cef_request_context_settings_t {
        let Self {
            cache_path,
            persist_session_cookies,
            accept_language_list,
            cookieable_schemes_exclude_defaults,
            cookieable_schemes_list,
        } = self;
        cef_request_context_settings_t {
            size: core::mem::size_of::<cef_request_context_settings_t>(),
            cache_path: cache_path.as_raw(),
            persist_session_cookies: *persist_session_cookies as _,
            accept_language_list: accept_language_list.as_raw(),
            cookieable_schemes_list: cookieable_schemes_list.as_raw(),
            cookieable_schemes_exclude_defaults: *cookieable_schemes_exclude_defaults as _,
        }
    }
}

/// See [cef_request_handler_t]
pub trait CefRequestHandler: Sized {
    /// See [cef_request_handler_t::on_before_browse]
    fn on_before_browse(
        &self,
        browser: CefBrowser,
        frame: CefFrame,
        request: CefRequest,
        user_gesture: ::std::os::raw::c_int,
        is_redirect: ::std::os::raw::c_int,
    ) -> bool {
        false
    }
    /// See [cef_request_handler_t::on_open_urlfrom_tab]
    fn on_open_urlfrom_tab(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        target_url: *const cef_string_t,
        target_disposition: cef_window_open_disposition_t,
        user_gesture: ::std::os::raw::c_int,
    ) -> bool {
        false
    }
    /// See [cef_request_handler_t::get_resource_request_handler]
    fn get_resource_request_handler(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        is_navigation: ::std::os::raw::c_int,
        is_download: ::std::os::raw::c_int,
        request_initiator: *const cef_string_t,
        disable_default_handling: *mut ::std::os::raw::c_int,
    ) -> *mut _cef_resource_request_handler_t;

    /// See [cef_request_handler_t::get_auth_credentials]
    fn get_auth_credentials(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        origin_url: *const cef_string_t,
        isProxy: ::std::os::raw::c_int,
        host: *const cef_string_t,
        port: ::std::os::raw::c_int,
        realm: *const cef_string_t,
        scheme: *const cef_string_t,
        callback: *mut _cef_auth_callback_t,
    ) -> ::std::os::raw::c_int;

    /// See [cef_request_handler_t::on_certificate_error]
    fn on_certificate_error(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        cert_error: cef_errorcode_t,
        request_url: *const cef_string_t,
        ssl_info: *mut _cef_sslinfo_t,
        callback: *mut _cef_callback_t,
    ) -> ::std::os::raw::c_int;

    /// See [cef_request_handler_t::on_select_client_certificate]
    fn on_select_client_certificate(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        isProxy: ::std::os::raw::c_int,
        host: *const cef_string_t,
        port: ::std::os::raw::c_int,
        certificatesCount: usize,
        certificates: *const *mut _cef_x509certificate_t,
        callback: *mut _cef_select_client_certificate_callback_t,
    ) -> ::std::os::raw::c_int;

    /// See [cef_request_handler_t::on_render_view_ready]
    fn on_render_view_ready(self_: *mut _cef_request_handler_t, browser: *mut _cef_browser_t);

    /// See [cef_request_handler_t::on_render_process_unresponsive]
    fn on_render_process_unresponsive(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        callback: *mut _cef_unresponsive_process_callback_t,
    ) -> ::std::os::raw::c_int;

    /// See [cef_request_handler_t::on_render_process_responsive]
    fn on_render_process_responsive(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
    );

    /// See [cef_request_handler_t::on_render_process_terminated]
    fn on_render_process_terminated(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        status: cef_termination_status_t,
        error_code: ::std::os::raw::c_int,
        error_string: *const cef_string_t,
    );

    /// See [cef_request_handler_t::on_document_available_in_main_frame]
    fn on_document_available_in_main_frame(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
    );

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_request_handler_t {
        unsafe extern "C" fn on_before_browse(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            user_gesture: ::std::os::raw::c_int,
            is_redirect: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int {
            false as _
        }

        unsafe extern "C" fn on_open_urlfrom_tab(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            target_url: *const cef_string_t,
            target_disposition: cef_window_open_disposition_t,
            user_gesture: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int {
            false as _
        }

        unsafe extern "C" fn get_resource_request_handler(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            is_navigation: ::std::os::raw::c_int,
            is_download: ::std::os::raw::c_int,
            request_initiator: *const cef_string_t,
            disable_default_handling: *mut ::std::os::raw::c_int,
        ) -> *mut _cef_resource_request_handler_t {
            std::ptr::null_mut()
        }

        unsafe extern "C" fn get_auth_credentials(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            origin_url: *const cef_string_t,
            isProxy: ::std::os::raw::c_int,
            host: *const cef_string_t,
            port: ::std::os::raw::c_int,
            realm: *const cef_string_t,
            scheme: *const cef_string_t,
            callback: *mut _cef_auth_callback_t,
        ) -> ::std::os::raw::c_int {
            false as _
        }

        unsafe extern "C" fn on_certificate_error(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            cert_error: cef_errorcode_t,
            request_url: *const cef_string_t,
            ssl_info: *mut _cef_sslinfo_t,
            callback: *mut _cef_callback_t,
        ) -> ::std::os::raw::c_int {
            false as _
        }

        unsafe extern "C" fn on_select_client_certificate(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            isProxy: ::std::os::raw::c_int,
            host: *const cef_string_t,
            port: ::std::os::raw::c_int,
            certificatesCount: usize,
            certificates: *const *mut _cef_x509certificate_t,
            callback: *mut _cef_select_client_certificate_callback_t,
        ) -> ::std::os::raw::c_int {
            false as _
        }

        unsafe extern "C" fn on_render_view_ready(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
        ) {
        }

        unsafe extern "C" fn on_render_process_unresponsive(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            callback: *mut _cef_unresponsive_process_callback_t,
        ) -> ::std::os::raw::c_int {
            false as _
        }

        unsafe extern "C" fn on_render_process_responsive(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
        ) {
        }

        unsafe extern "C" fn on_render_process_terminated(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            status: cef_termination_status_t,
            error_code: ::std::os::raw::c_int,
            error_string: *const cef_string_t,
        ) {
        }

        unsafe extern "C" fn on_document_available_in_main_frame(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
        ) {
        }

        let mut object: cef_request_handler_t = unsafe { std::mem::zeroed() };
        crate::rc::RcImpl::new(object, self).cast()
    }
}

/// See [cef_request_context_handler_t]
pub trait CefRequestContextHandler: Sized {
    /// See [cef_request_context_handler_t::on_request_context_initialized]
    fn on_request_context_initialized(&self, request_context: CefRequestContext) {}

    /// See [cef_request_context_handler_t::get_resource_request_handler]
    fn get_resource_request_handler(
        self_: *mut _cef_request_context_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        is_navigation: ::std::os::raw::c_int,
        is_download: ::std::os::raw::c_int,
        request_initiator: *const cef_string_t,
        disable_default_handling: *mut ::std::os::raw::c_int,
    ) -> *mut _cef_resource_request_handler_t {
        std::ptr::null_mut()
    }

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_request_context_handler_t {
        unsafe extern "C" fn on_request_context_initialized<I: CefRequestContextHandler>(
            self_: *mut _cef_request_context_handler_t,
            request_context: *mut cef_request_context_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            object
                .interface
                .on_request_context_initialized(CefRequestContext::from(request_context));
        }

        let mut object: cef_request_context_handler_t = unsafe { std::mem::zeroed() };
        object.on_request_context_initialized = Some(on_request_context_initialized::<Self>);
        crate::rc::RcImpl::new(object, self).cast()
    }
}
