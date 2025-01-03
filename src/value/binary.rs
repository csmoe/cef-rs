use std::os::raw::c_void;

use crate::prelude::*;

/// See [cef_binary_value_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefBinaryValue(cef_binary_value_t);

impl CefBinaryValue {
    /// See [cef_binary_value_create]
    pub fn create(data: &[u8]) -> Result<CefBinaryValue> {
        let ptr = unsafe { cef_binary_value_create(data.as_ptr().cast(), data.len()) };
        if ptr.is_null() {
            Err(crate::error::Error::NullPtr)
        } else {
            Ok(CefBinaryValue::from(ptr))
        }
    }

    pub fn get_inner_data(&self) -> &[u8] {
        if let Some(ptr) = self.get_raw_data() {
            if ptr.is_null() {
                return &mut [];
            }
            if let Some(len) = self.get_size() {
                return unsafe { std::slice::from_raw_parts(ptr.cast(), len) };
            }
        }
        &mut []
    }

    wrapper_methods! {
        /// See [cef_binary_value_t::get_raw_data]
        fn get_raw_data(&self) -> *const c_void;

        /// See [cef_binary_value_t::get_size]
        fn get_size(&self) -> usize;
    }
}
