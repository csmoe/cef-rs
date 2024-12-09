use crate::prelude::*;

/// See [cef_dictionary_value_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefDictionaryValue(cef_dictionary_value_t);

impl CefDictionaryValue {
    pub fn create() -> Result<CefDictionaryValue> {
        let ptr = unsafe { cef_dictionary_value_create() };
        if ptr.is_null() {
            Err(crate::error::Error::NullPtr)
        } else {
            Ok(unsafe { CefDictionaryValue::from_raw(ptr) })
        }
    }
}
