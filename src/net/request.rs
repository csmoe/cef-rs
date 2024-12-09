use crate::prelude::*;

/// See [cef_request_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct Request(cef_request_t);

impl Request {
    pub fn create() -> Result<Request> {
        let ptr = unsafe { cef_request_create() };
        if ptr.is_null() {
            Err(crate::error::Error::NullPtr)
        } else {
            Ok(unsafe { Request::from_raw(ptr) })
        }
    }
}

impl Request {
    wrapper_methods!(
        /// See [cef_request_t::is_read_only]
        fn is_read_only(&self) -> bool;

        /// See [cef_request_t::get_url]
        fn get_url(&self) -> CefString {
            self.0
                .get_url
                .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.0.get_this())) })
        }

        /// See [cef_request_t::set_url]
        fn set_url(&self, url: &str) {
            self.0
                .set_url
                .map(|f| unsafe { f(self.0.get_this(), &CefString::from(url).as_raw()) })
        }

        /// See [cef_request_t::get_method]
        fn get_method(&self) -> CefString {
            self.0
                .get_method
                .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.0.get_this())) })
        }

        /// See [cef_request_t::set_method]
        fn set_method(&self, method: &str) {
            self.0
                .set_method
                .map(|f| unsafe { f(self.0.get_this(), &CefString::from(method).as_raw()) })
        }

        /// See [cef_request_t::set_referrer]
        fn set_referrer(&self, referrer_url: &str, policy: cef_referrer_policy_t) {
            self.0.set_referrer.map(|f| unsafe {
                f(
                    self.0.get_this(),
                    &CefString::from(referrer_url).as_raw(),
                    policy as _,
                )
            })
        }

        /// See [cef_request_t::get_referrer_url]
        fn get_referrer_url(&self) -> CefString {
            self.0
                .get_referrer_url
                .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.0.get_this())) })
        }

        /// See [cef_request_t::get_referrer_policy]
        fn get_referrer_policy(&self) -> cef_referrer_policy_t;

        /// See [cef_request_t::get_post_data]
        fn get_post_data(&self) -> crate::net::PostData {
            self.0.get_post_data.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() {
                    return None;
                }
                crate::net::PostData::from_raw(v).into()
            })
        }

        /// See [cef_request_t::set_post_data]
        fn set_post_data(&self, post_data: crate::net::PostData);

        /// See [cef_request_t::get_header_map]
        fn get_header_map(&self) -> crate::multimap::CefStringMultiMap {
            self.0.get_header_map.map(|f| unsafe {
                let map = std::ptr::null_mut();
                f(self.0.get_this(), map);
                crate::multimap::CefStringMultiMap::from_raw(map)
            })
        }

        /// See [cef_request_t::set_header_map]
        fn set_header_map(&self, header_map: crate::multimap::CefStringMultiMap) {
            self.0
                .set_header_map
                .map(|f| unsafe { f(self.0.get_this(), header_map.as_raw()) })
        }

        /// See [cef_request_t::get_header_by_name]
        fn get_header_by_name(&self, name: &str) -> CefString {
            self.0.get_header_by_name.and_then(|f| unsafe {
                CefString::from_userfree_cef(f(self.0.get_this(), &CefString::from(name).as_raw()))
            })
        }

        /// See [cef_request_t::set_header_by_name]
        fn set_header_by_name(&self, name: &str, value: &str, overwrite: bool) {
            self.0.set_header_by_name.map(|f| unsafe {
                f(
                    self.0.get_this(),
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
            post_data: crate::net::PostData,
            header_map: crate::multimap::CefStringMultiMap,
        ) {
            self.0.set.map(|f| unsafe {
                f(
                    self.0.get_this(),
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
            self.0
                .get_first_party_for_cookies
                .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.0.get_this())) })
        }

        /// See [cef_request_t::set_first_party_for_cookies]
        fn set_first_party_for_cookies(&self, url: &str) {
            self.0
                .set_first_party_for_cookies
                .map(|f| unsafe { f(self.0.get_this(), &CefString::from(url).as_raw()) })
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
pub struct RequestContext(cef_request_context_t);

impl RequestContext {
    pub fn global() -> Self {
        unsafe { RequestContext::from_raw(cef_request_context_get_global_context()) }
    }
}
