use crate::prelude::*;

wrapper! {
    /// See [cef_binary_value_t] for more docs.
    #[derive(Debug, Clone)]
    pub struct BinaryValue(cef_binary_value_t);
}

impl BinaryValue {
    fn create(data: &[u8]) -> crate::Result<BinaryValue> {
        let ptr = unsafe { cef_binary_value_create(data.as_ptr().cast(), data.len()) };
        if ptr.is_null() {
            Err(crate::error::Error::NullPtr)
        } else {
            Ok(unsafe { BinaryValue::from_raw(ptr) })
        }
    }
}
