use crate::prelude::*;

wrapper! {
    /// See [cef_dictionary_value_t] for more docs.
    #[derive(Debug, Clone)]
    pub struct DictionaryValue(cef_dictionary_value_t);
}

impl DictionaryValue {
    pub fn create() -> Result<DictionaryValue> {
        let ptr = unsafe { cef_dictionary_value_create() };
        if ptr.is_null() {
            Err(crate::error::Error::NullPtr)
        } else {
            Ok(unsafe { DictionaryValue::from_raw(ptr) })
        }
    }
}
