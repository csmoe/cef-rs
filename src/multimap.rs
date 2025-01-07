use cef_sys::{
    _cef_string_multimap_t, cef_string_multimap_alloc, cef_string_multimap_append,
    cef_string_multimap_clear, cef_string_multimap_enumerate, cef_string_multimap_find_count,
    cef_string_multimap_free, cef_string_multimap_key, cef_string_multimap_size,
    cef_string_multimap_value,
};
use std::ptr::NonNull;

use crate::CefString;

pub struct CefStringMultiMap {
    ptr: Option<NonNull<_cef_string_multimap_t>>,
}

impl Drop for CefStringMultiMap {
    fn drop(&mut self) {
        let Some(map) = self.ptr else { return };
        unsafe {
            cef_string_multimap_free(map.as_ptr());
        }
    }
}

impl Default for CefStringMultiMap {
    fn default() -> Self {
        Self::new()
    }
}

impl CefStringMultiMap {
    /// See [cef_string_multimap_alloc]
    pub fn new() -> Self {
        let ptr = unsafe { NonNull::new(cef_string_multimap_alloc()) };
        Self { ptr }
    }

    pub fn from_raw(ptr: cef_sys::cef_string_multimap_t) -> Self {
        Self {
            ptr: NonNull::new(ptr),
        }
    }

    pub fn as_raw(&self) -> cef_sys::cef_string_multimap_t {
        let Some(map) = self.ptr else {
            return std::ptr::null_mut();
        };
        map.as_ptr()
    }

    /// See [cef_string_multimap_size]
    pub fn len(&self) -> usize {
        let Some(map) = self.ptr else { return 0 };
        unsafe { cef_string_multimap_size(map.as_ptr()) }
    }

    /// See [cef_string_multimap_clear]
    pub fn clear(&mut self) {
        let Some(map) = self.ptr else { return };
        unsafe {
            cef_string_multimap_clear(map.as_ptr());
        }
    }

    /// See [cef_string_multimap_append]
    pub fn append(&mut self, key: &str, value: &str) -> bool {
        let Some(map) = self.ptr else { return false };
        unsafe {
            cef_string_multimap_append(
                map.as_ptr(),
                &CefString::from(key).as_raw(),
                &CefString::from(value).as_raw(),
            ) == 1
        }
    }

    /// See [cef_string_multimap_find_count]
    pub fn find_count(&self, key: &str) -> usize {
        let Some(map) = self.ptr else { return 0 };
        unsafe { cef_string_multimap_find_count(map.as_ptr(), &CefString::from(key).as_raw()) }
    }

    /// See [cef_string_multimap_key]
    pub fn key(&self, index: usize) -> Option<CefString> {
        let Some(map) = self.ptr else { return None };
        let key = std::ptr::null_mut();
        unsafe {
            if cef_string_multimap_key(map.as_ptr(), index, key) == 1 {
                CefString::from_raw(key)
            } else {
                None
            }
        }
    }

    /// See [cef_string_multimap_value]
    pub fn value(&self, index: usize) -> Option<CefString> {
        let Some(map) = self.ptr else { return None };
        let value = std::ptr::null_mut();
        unsafe {
            if cef_string_multimap_value(map.as_ptr(), index, value) == 1 {
                CefString::from_raw(value)
            } else {
                None
            }
        }
    }

    /// See [cef_string_multimap_enumerate]
    pub fn enumerate(&self, key: &str, index: usize) -> Option<CefString> {
        let Some(map) = self.ptr else { return None };
        let value = std::ptr::null_mut();
        unsafe {
            if cef_string_multimap_enumerate(
                map.as_ptr(),
                &CefString::from(key).as_raw(),
                index,
                value,
            ) == 1
            {
                CefString::from_raw(value)
            } else {
                None
            }
        }
    }
}
