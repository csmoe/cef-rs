use crate::prelude::*;

/// See [cef_binary_value_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefBinaryValue(cef_binary_value_t);

impl CefBinaryValue {
    pub fn create(data: &[u8]) -> Result<CefBinaryValue> {
        let ptr = unsafe { cef_binary_value_create(data.as_ptr().cast(), data.len()) };
        if ptr.is_null() {
            Err(crate::error::Error::NullPtr)
        } else {
            Ok(unsafe { CefBinaryValue::from(ptr) })
        }
    }
}
