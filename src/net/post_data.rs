use crate::prelude::*;

/// See [cef_post_data_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefPostData(cef_post_data_t);

impl CefPostData {
    /// See [cef_post_data_create].
    pub fn create() -> Result<CefPostData> {
        let ptr = unsafe { cef_post_data_create() };
        if ptr.is_null() {
            Err(Error::NullPtr)
        } else {
            Ok(unsafe { CefPostData::from_raw(ptr) })
        }
    }
}

/// See [cef_post_data_element_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefPostDataElement(cef_post_data_element_t);

impl CefPostDataElement {
    pub fn create() -> Result<CefPostDataElement> {
        let ptr = unsafe { cef_post_data_element_create() };
        if ptr.is_null() {
            Err(Error::NullPtr)
        } else {
            Ok(unsafe { CefPostDataElement::from_raw(ptr) })
        }
    }
}

impl CefPostDataElement {
    wrapper_methods! {
        /// See [cef_post_data_element_t::is_read_only].
        fn is_read_only(&self) -> bool;

        /// See [cef_post_data_element_t::set_to_empty].
        fn set_to_empty(&mut self);

        /// See [cef_post_data_element_t::set_to_file].
        fn set_to_file(&mut self, file_path: &std::path::Path) {
            self.0.set_to_file.map(|f| unsafe {f(self.0.get_this(), &CefString::from(file_path).as_raw())})
        }

        /// See [cef_post_data_element_t::set_to_bytes].
        fn set_to_bytes(&mut self, bytes: &[u8]) {
            self.0.set_to_bytes.map(|f| unsafe {f(self.0.get_this(),  bytes.len() , bytes.as_ptr().cast(),)})
        }

        /// See [cef_post_data_element_t::get_type].
        fn get_type(&self) -> crate::CefPostDataElementType {
            self.0.get_type.map(|f| unsafe {f(self.0.get_this())})
        }

        /// See [cef_post_data_element_t::get_file].
        fn get_file(&self) -> CefString {
            self.0.get_file.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() {  None  } else {  CefString::from_raw(v) }
            })
        }

        /// See [cef_post_data_element_t::get_bytes].
        fn get_bytes(&self, size: usize) -> Vec<u8> {
            let mut bytes = vec![0; size];
            self.0.get_bytes.map(|f| unsafe {
                f(self.0.get_this(), size, bytes.as_mut_ptr().cast());
                bytes
            })
        }

        /// See [cef_post_data_element_t::get_bytes_count].
        fn get_bytes_count(&self) -> usize {
                self.0.get_bytes_count.map(|f| unsafe {f(self.0.get_this())})
        }
    }
}
