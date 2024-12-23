use crate::prelude::*;

/// See [cef_list_value_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefListValue(cef_list_value_t);

impl CefListValue {
    pub fn create() -> Result<CefListValue> {
        let ptr = unsafe { cef_list_value_create() };
        if ptr.is_null() {
            Err(crate::error::Error::NullPtr)
        } else {
            Ok(CefListValue::from(ptr))
        }
    }

    wrapper_methods!(
        /// See [cef_list_value_t::is_valid]
        fn is_valid(&self) -> bool;

        /// See [cef_list_value_t::is_owned]
        fn is_owned(&self) -> bool;

        /// See [cef_list_value_t::is_read_only]
        fn is_read_only(&self) -> bool;

        /// See [cef_list_value_t::is_same]
        fn is_same(&self, that: CefListValue) -> bool {
            is_same.map(|f| unsafe { f(self.get_this(), that.into_raw()) == 1 })
        }

        /// See [cef_list_value_t::is_equal]
        fn is_equal(&self, that: CefListValue) -> bool {
            is_equal.map(|f| unsafe { f(self.get_this(), that.into_raw()) == 1 })
        }

        /// See [cef_list_value_t::copy]
        fn copy(&self) -> CefListValue {
            copy.and_then(|f| unsafe {
                let l = f(self.get_this());
                if l.is_null() {
                    None
                } else {
                    Some(CefListValue::from(l))
                }
            })
        }

        /// See [cef_list_value_t::set_size]
        fn set_size(&self, size: usize) -> bool;

        /// See [cef_list_value_t::get_size]
        fn get_size(&self) -> usize;

        /// See [cef_list_value_t::clear]
        fn clear(&self) -> bool;

        /// See [cef_list_value_t::remove]
        fn remove(&self, index: usize) -> bool;

        /// See [cef_list_value_t::get_type]
        fn get_type(&self, index: usize) -> crate::CefValueType;

        /// See [cef_list_value_t::get_value]
        fn get_value(&self, index: usize) -> crate::CefValue {
            get_value.and_then(|f| unsafe {
                let v = f(self.get_this(), index);
                if v.is_null() {
                    None
                } else {
                    Some(crate::CefValue::from(v))
                }
            })
        }

        /// See [cef_list_value_t::get_bool]
        fn get_bool(&self, index: usize) -> bool;

        /// See [cef_list_value_t::get_int]
        fn get_int(&self, index: usize) -> i32;

        /// See [cef_list_value_t::get_double]
        fn get_double(&self, index: usize) -> f64;

        /// See [cef_list_value_t::get_string]
        fn get_string(&self, index: usize) -> CefString {
            get_string
                .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.get_this(), index)) })
        }

        /// See [cef_list_value_t::get_binary]
        fn get_binary(&self, index: usize) -> crate::CefBinaryValue {
            get_binary.and_then(|f| unsafe {
                let v = f(self.get_this(), index);
                if v.is_null() {
                    None
                } else {
                    Some(crate::CefBinaryValue::from(v))
                }
            })
        }

        /// See [cef_list_value_t::get_dictionary]
        fn get_dictionary(&self, index: usize) -> crate::CefDictionaryValue {
            get_dictionary.and_then(|f| unsafe {
                let v = f(self.get_this(), index);
                if v.is_null() {
                    None
                } else {
                    Some(crate::CefDictionaryValue::from(v))
                }
            })
        }

        /// See [cef_list_value_t::get_list]
        fn get_list(&self, index: usize) -> CefListValue {
            get_list.and_then(|f| unsafe {
                let v = f(self.get_this(), index);
                if v.is_null() {
                    None
                } else {
                    Some(CefListValue::from(v))
                }
            })
        }

        /// See [cef_list_value_t::set_value]
        fn set_value(&self, index: usize, value: crate::CefValue) -> bool {
            set_value.map(|f| unsafe { f(self.get_this(), index, value.into_raw()) == 1 })
        }

        /// See [cef_list_value_t::set_null]
        fn set_null(&self, index: usize) -> bool;

        /// See [cef_list_value_t::set_bool]
        fn set_bool(&self, index: usize, value: bool) -> bool;

        /// See [cef_list_value_t::set_int]
        fn set_int(&self, index: usize, value: i32) -> bool;

        /// See [cef_list_value_t::set_double]
        fn set_double(&self, index: usize, value: f64) -> bool;

        /// See [cef_list_value_t::set_string]
        fn set_string(&self, index: usize, value: &str) -> bool {
            set_string.map(|f| unsafe {
                f(self.get_this(), index, &CefString::from(value).as_raw()) == 1
            })
        }

        /// See [cef_list_value_t::set_binary]
        fn set_binary(&self, index: usize, value: crate::CefBinaryValue) -> bool;

        /// See [cef_list_value_t::set_dictionary]
        fn set_dictionary(&self, index: usize, value: crate::CefDictionaryValue) -> bool;

        /// See [cef_list_value_t::set_list]
        fn set_list(&self, index: usize, value: CefListValue) -> bool;
    );
}
