//! String module
//!
//! cef-rs defines its own type [`CefString`] which is a new
//! type of [`widestring::U16CString`] to handle everything around strings.
//! Including converting from/to raw [`cef_string_utf16_t`],
//! converting from/to Rust string types. Every cef-rs types should also use [`CefString`] as
//! interface in most of the cases. Raw cef string [`cef_string_utf16_t`] is a UTF-16 C String,
//! but it also has a version [`cef_string_userfree_utf16_t`] that users are responsible
//! for free it manually.

use cef_sys::{
    cef_string_list_t, cef_string_map_t, cef_string_userfree_utf16_t, cef_string_utf16_t,
};
use std::collections::HashMap;
use std::ptr::null_mut;
use widestring::U16CString;

/// Helper type to deal with Cef string. It's essentially an UTF-16 C string.
#[derive(Debug, Default, Clone, Hash)]
pub struct CefString(U16CString);

impl CefString {
    pub fn new(s: &str) -> Self {
        Self(U16CString::from_str(s).expect("Failed to create CefString from str."))
    }

    /// Create a `CefString` from raw `cef_string_utf16_t` pointer. If the pointer is null or it fails
    /// to convert to `U16CString`, this method will returns `None`.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from_raw(ptr: *const cef_string_utf16_t) -> Option<CefString> {
        if ptr.is_null() {
            None
        } else {
            // It's a smart pointer, so cef retains ownership and will call the dtor
            unsafe {
                U16CString::from_ptr((*ptr).str_, (*ptr).length)
                    .ok()
                    .map(CefString)
            }
        }
    }

    /// Create a `CefString` from raw `cef_string_userfree_utf16_t` pointer. If the pointer is null or it fails
    /// to convert to `U16CString`, this method will returns `None`.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from_userfree_cef(ptr: cef_string_userfree_utf16_t) -> Option<CefString> {
        if ptr.is_null() {
            return None;
        }
        unsafe {
            let res = Self::from_raw(ptr);
            cef_sys::cef_string_userfree_utf16_free(ptr);
            res
        }
    }

    /// Get raw [cef_string_utf16_t] which doesn't have the ownership of the value.
    /// This should be used when you need to pass the `*const cef_string_utf16_t` to the function.
    pub fn as_raw(&self) -> cef_string_utf16_t {
        unsafe extern "C" fn free_string(str_: *mut cef_sys::char16_t) {
            if str_.is_null() {
                return;
            }
            _ = U16CString::from_raw(str_);
        }
        cef_string_utf16_t {
            length: self.0.len(),
            str_: self.0.as_ptr().cast_mut(),
            dtor: None,
        }
    }
}

impl<T: AsRef<std::ffi::OsStr>> From<T> for CefString {
    fn from(value: T) -> Self {
        Self(U16CString::from_os_str(value.as_ref()).expect("cannot convert OsStr to U16CString"))
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for CefString {
    fn to_string(&self) -> String {
        self.0.to_string_lossy()
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn parse_string_list(ptr: cef_string_list_t) -> Vec<CefString> {
    let count = cef_sys::cef_string_list_size(ptr);
    let mut res = Vec::with_capacity(count);
    for i in 0..count {
        let value = null_mut();
        if cef_sys::cef_string_list_value(ptr, i, value) > 0 {
            if let Some(v) = CefString::from_raw(value) {
                res.push(v)
            }
        }
    }
    res
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn parse_string_map(ptr: cef_string_map_t) -> HashMap<String, String> {
    let count = cef_sys::cef_string_map_size(ptr);
    let mut res = HashMap::with_capacity(count);
    for i in 0..count {
        let key = null_mut();
        let value = null_mut();
        cef_sys::cef_string_map_key(ptr, i, key);
        cef_sys::cef_string_map_value(ptr, i, value);

        CefString::from_raw(key)
            .map(|k| CefString::from_raw(value).map(|v| res.insert(k.to_string(), v.to_string())));
    }
    res
}
