use crate::prelude::*;

/// See [cef_process_message_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefProcessMessage(cef_process_message_t);

impl CefProcessMessage {
    /// See [cef_process_message_create]
    pub fn create(name: &str) -> Result<CefProcessMessage> {
        let name = CefString::from(name);
        unsafe {
            let msg = cef_process_message_create(&name.as_raw());
            if msg.is_null() {
                return Err(Error::NullPtr);
            }
            Ok(CefProcessMessage::from(msg))
        }
    }

    wrapper_methods!(
        /// See [cef_process_message_t::is_valid]
        fn is_valid(&self) -> bool;

        /// See [cef_process_message_t::is_read_only]
        fn is_read_only(&self) -> bool;

        /// See [cef_process_message_t::copy]
        fn copy(&self) -> CefProcessMessage {
            if !self.is_valid().unwrap_or_default() {
                return None;
            }
            copy.and_then(|f| unsafe {
                let v = f(self.get_this());
                if v.is_null() {
                    None
                } else {
                    Some(CefProcessMessage::from(v))
                }
            })
        }

        /// See [cef_process_message_t::get_name]
        fn get_name(&self) -> CefString {
            if !self.is_valid().unwrap_or_default() {
                return None;
            }
            get_name.and_then(|f| unsafe { CefString::from_userfree_cef(f(self.get_this())) })
        }

        /// See [cef_process_message_t::get_argument_list]
        fn get_argument_list(&self) -> crate::CefListValue {
            if !self.is_valid().unwrap_or_default() {
                return None;
            }
            get_argument_list.and_then(|f| unsafe {
                let ptr = f(self.get_this());
                if ptr.is_null() {
                    None
                } else {
                    Some(crate::CefListValue::from(ptr))
                }
            })
        }
    );
    // TODO
    // See [cef_process_message_t::get_shared_memory_region]
    //fn get_shared_memory_region(&self) -> cef_sys::cef_shared_memory_region_t;
}
