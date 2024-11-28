use crate::prelude::*;

wrapper! {
    /// See [cef_list_value_t] for more docs.
    #[derive(Debug, Clone)]
    pub struct ListValue(cef_list_value_t);
}

impl ListValue {
    pub fn create() -> Result<ListValue> {
        let ptr = unsafe { cef_list_value_create() };
        if ptr.is_null() {
            Err(crate::error::Error::NullPtr)
        } else {
            Ok(unsafe { ListValue::from_raw(ptr) })
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
        fn is_same(&self, that: ListValue) -> bool {
            self.0
                .is_same
                .map(|f| unsafe { f(self.0.get_this(), that.0.into_raw()) == 1 })
        }

        /// See [cef_list_value_t::is_equal]
        fn is_equal(&self, that: ListValue) -> bool {
            self.0
                .is_equal
                .map(|f| unsafe { f(self.0.get_this(), that.0.into_raw()) == 1 })
        }

        /// See [cef_list_value_t::copy]
        fn copy(&self) -> ListValue {
            self.0.copy.and_then(|f| unsafe {
                let l = f(self.0.get_this());
                if l.is_null() {
                    None
                } else {
                    Some(ListValue::from_raw(l))
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
        fn get_type(&self, index: usize) -> crate::ValueType;

        /// See [cef_list_value_t::get_value]
        fn get_value(&self, index: usize) -> crate::Value {
            self.0.get_value.and_then(|f| unsafe {
                let v = f(self.0.get_this(), index);
                if v.is_null() {
                    None
                } else {
                    Some(crate::Value::from_raw(v))
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
            self.0
                .get_string
                .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.0.get_this(), index)) })
        }

        /// See [cef_list_value_t::get_binary]
        fn get_binary(&self, index: usize) -> crate::BinaryValue {
            self.0.get_binary.and_then(|f| unsafe {
                let v = f(self.0.get_this(), index);
                if v.is_null() {
                    None
                } else {
                    Some(crate::BinaryValue::from_raw(v))
                }
            })
        }

        /// See [cef_list_value_t::get_dictionary]
        fn get_dictionary(&self, index: usize) -> crate::DictionaryValue {
            self.0.get_dictionary.and_then(|f| unsafe {
                let v = f(self.0.get_this(), index);
                if v.is_null() {
                    None
                } else {
                    Some(crate::DictionaryValue::from_raw(v))
                }
            })
        }

        /// See [cef_list_value_t::get_list]
        fn get_list(&self, index: usize) -> ListValue {
            self.0.get_list.and_then(|f| unsafe {
                let v = f(self.0.get_this(), index);
                if v.is_null() {
                    None
                } else {
                    Some(ListValue::from_raw(v))
                }
            })
        }

        /// See [cef_list_value_t::set_value]
        fn set_value(&self, index: usize, value: crate::Value) -> bool {
            self.0
                .set_value
                .map(|f| unsafe { f(self.0.get_this(), index, value.into_raw()) == 1 })
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
            self.0.set_string.map(|f| unsafe {
                f(
                    self.0.get_this(),
                    index,
                    std::ptr::from_ref(&<_ as Into<CefString>>::into(value).as_raw()),
                ) == 1
            })
        }

        /// See [cef_list_value_t::set_binary]
        fn set_binary(&self, index: usize, value: crate::BinaryValue) -> bool;

        /// See [cef_list_value_t::set_dictionary]
        fn set_dictionary(&self, index: usize, value: crate::DictionaryValue) -> bool;

        /// See [cef_list_value_t::set_list]
        fn set_list(&self, index: usize, value: ListValue) -> bool;
    );
}
