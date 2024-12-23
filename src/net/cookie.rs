use crate::{prelude::*, CefBaseTime, CefCookiePriority, CefCookieSameSite};

#[derive(Debug, Clone)]
pub struct CefCookie {
    #[doc = "\n The cookie name.\n"]
    pub name: CefString,
    #[doc = "\n The cookie value.\n"]
    pub value: CefString,
    #[doc = "\n If |domain| is empty a host cookie will be created instead of a domain\n cookie. Domain cookies are stored with a leading \".\" and are visible to\n sub-domains whereas host cookies are not.\n"]
    pub domain: CefString,
    #[doc = "\n If |path| is non-empty only URLs at or below the path will get the cookie\n value.\n"]
    pub path: CefString,
    #[doc = "\n If |secure| is true the cookie will only be sent for HTTPS requests.\n"]
    pub secure: bool,
    #[doc = "\n If |httponly| is true the cookie will only be sent for HTTP requests.\n"]
    pub httponly: bool,
    #[doc = "\n The cookie creation date. This is automatically populated by the system on\n cookie creation.\n"]
    pub creation: CefBaseTime,
    #[doc = "\n The cookie last access date. This is automatically populated by the system\n on access.\n"]
    pub last_access: CefBaseTime,
    #[doc = "\n The cookie expiration date is only valid if |has_expires| is true.\n"]
    pub has_expires: bool,
    pub expires: CefBaseTime,
    #[doc = "\n Same site.\n"]
    pub same_site: CefCookieSameSite,
    #[doc = "\n Priority.\n"]
    pub priority: CefCookiePriority,
}

impl CefCookie {
    fn into_raw(self) -> cef_cookie_t {
        let Self {
            name,
            value,
            domain,
            path,
            secure,
            httponly,
            creation,
            last_access,
            has_expires,
            expires,
            same_site,
            priority,
        } = self;

        cef_cookie_t {
            name: name.as_raw(),
            value: value.as_raw(),
            domain: domain.as_raw(),
            path: path.as_raw(),
            secure: secure as _,
            httponly: httponly as _,
            creation,
            last_access,
            has_expires: has_expires as _,
            expires,
            same_site,
            priority,
        }
    }
}

#[wrapper]
#[derive(Debug, Clone)]
pub struct CefCookieManager(cef_cookie_manager_t);

impl CefCookieManager {
    wrapper_methods! {
        /// See [cef_cookie_manager_t::set_cookie]
        fn set_cookie(&self,
            url: CefString,
            cookie: CefCookie,
            callback: impl CefSetCookieCallback,
        ) -> bool {
            set_cookie.map(|f|unsafe {
                f(self.get_this(), &url.as_raw(), &cookie.into_raw(), callback.into_raw()) == 1
            })
        }

        /// See [cef_cookie_manager_t::delete_cookies]
        fn delete_cookies(
            &self,
            url: CefString,
            cookie_name: CefString,
            callback: impl CefDeleteCookiesCallback,
        ) -> bool {
            delete_cookies.map(|f| unsafe {
                f(self.get_this(), &url.as_raw(), &cookie_name.as_raw(), callback.into_raw())  == 1
            })
        }
    }
}

/// See [cef_set_cookie_callback_t]
pub trait CefSetCookieCallback: Sized {
    /// See [cef_set_cookie_callback_t::on_complete]
    fn on_complete(&self, success: bool);

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_set_cookie_callback_t {
        unsafe extern "C" fn on_complete<I: CefSetCookieCallback>(
            self_: *mut _cef_set_cookie_callback_t,
            success: ::std::os::raw::c_int,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            object.interface.on_complete(success == 1);
        }

        let mut object: cef_set_cookie_callback_t = unsafe { std::mem::zeroed() };
        object.on_complete = Some(on_complete::<Self>);
        crate::rc::RcImpl::new(object, self).cast()
    }
}

/// See [cef_delete_cookies_callback_t]
pub trait CefDeleteCookiesCallback: Sized {
    /// See [cef_delete_cookies_callback_t::on_complete]
    fn on_complete(&self, num_deleted: u32);

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_delete_cookies_callback_t {
        unsafe extern "C" fn on_complete<I: CefDeleteCookiesCallback>(
            self_: *mut _cef_delete_cookies_callback_t,
            num_deleted: ::std::os::raw::c_int,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            object.interface.on_complete(num_deleted as _);
        }

        let mut object: cef_delete_cookies_callback_t = unsafe { std::mem::zeroed() };
        object.on_complete = Some(on_complete::<Self>);
        crate::rc::RcImpl::new(object, self).cast()
    }
}
