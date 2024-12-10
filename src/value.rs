use crate::prelude::*;

mod binary;
mod dictionary;
mod list;

pub use binary::*;
pub use dictionary::*;
pub use list::*;

/// See [cef_value_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefValue(cef_value_t);

impl CefValue {
    pub fn create() -> Result<CefValue> {
        let ptr = unsafe { cef_value_create() };
        if ptr.is_null() {
            Err(Error::NullPtr)
        } else {
            Ok(unsafe { CefValue::from_raw(ptr) })
        }
    }
}

impl CefValue {
    wrapper_methods!(
        /// See [cef_value_t::is_valid]
        fn is_valid(&self) -> bool;

        /// See [cef_value_t::is_owned]
        fn is_owned(&self) -> bool;

        /// See [cef_value_t::is_read_only]
        fn is_read_only(&self) -> bool;

        /// See [cef_value_t::is_same]
        fn is_same(&self, that: CefValue) -> bool {
            self.0
                .is_same
                .map(|f| unsafe { f(self.0.get_this(), that.0.into_raw()) == 1 })
        }

        /// See [cef_value_t::is_equal]
        fn is_equal(&self, that: CefValue) -> bool {
            self.0
                .is_equal
                .map(|f| unsafe { f(self.0.get_this(), that.0.into_raw()) == 1 })
        }

        /// See [cef_value_t::copy]
        fn copy(&self) -> CefValue {
            self.0.copy.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() {
                    None
                } else {
                    Some(CefValue::from_raw(v))
                }
            })
        }

        /// See [cef_value_t::get_type]
        fn get_type(&self) -> crate::CefValueType;

        /// See [cef_value_t::get_bool]
        fn get_bool(&self) -> bool;

        /// See [cef_value_t::get_int]
        fn get_int(&self) -> i32;

        /// See [cef_value_t::get_double]
        fn get_double(&self) -> f64;

        /// See [cef_value_t::get_string]
        fn get_string(&self) -> CefString {
            self.0
                .get_string
                .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.0.get_this())) })
        }

        /// See [cef_value_t::get_binary]
        fn get_binary(&self) -> crate::CefBinaryValue {
            self.0.get_binary.and_then(|f| unsafe {
                let ptr = f(self.0.get_this());
                if ptr.is_null() {
                    None
                } else {
                    Some(crate::CefBinaryValue::from_raw(ptr))
                }
            })
        }

        /// See [cef_value_t::get_dictionary]
        fn get_dictionary(&self) -> crate::CefDictionaryValue {
            self.0.get_dictionary.and_then(|f| unsafe {
                let ptr = f(self.0.get_this());
                if ptr.is_null() {
                    None
                } else {
                    Some(crate::CefDictionaryValue::from_raw(ptr))
                }
            })
        }

        /// See [cef_value_t::get_list]
        fn get_list(&self) -> crate::CefListValue {
            self.0.get_list.and_then(|f| unsafe {
                let ptr = f(self.0.get_this());
                if ptr.is_null() {
                    None
                } else {
                    Some(crate::CefListValue::from_raw(ptr))
                }
            })
        }

        /// See [cef_value_t::set_null]
        fn set_null(&self) -> bool;

        /// See [cef_value_t::set_bool]
        fn set_bool(&self, value: bool) -> bool;

        /// See [cef_value_t::set_int]
        fn set_int(&self, value: i32) -> bool;

        /// See [cef_value_t::set_double]
        fn set_double(&self, value: f64) -> bool;

        /// See [cef_value_t::set_string]
        fn set_string(&self, value: &str) -> bool {
            self.0
                .set_string
                .map(|f| unsafe { f(self.0.get_this(), &CefString::from(value).as_raw()) == 1 })
        }

        /// See [cef_value_t::set_binary]
        fn set_binary(&self, value: crate::CefBinaryValue) -> bool;

        /// See [cef_value_t::set_dictionary]
        fn set_dictionary(&self, value: crate::CefDictionaryValue) -> bool {
            self.0
                .set_dictionary
                .map(|f| unsafe { f(self.0.get_this(), value.into_raw()) == 1 })
        }

        /// See [cef_value_t::set_list]
        fn set_list(&self, value: crate::CefListValue) -> bool {
            self.0
                .set_list
                .map(|f| unsafe { f(self.0.get_this(), value.into_raw()) == 1 })
        }
    );
}
