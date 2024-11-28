use crate::prelude::*;

wrapper! {
    /// See [cef_process_message_t] for more docs.
    #[derive(Debug, Clone)]
    pub struct ProcessMessage(cef_process_message_t);
}

impl ProcessMessage {
    wrapper_methods!(
        /// See [cef_process_message_t::is_valid]
        fn is_valid(&self) -> bool;

        /// See [cef_process_message_t::is_read_only]
        fn is_read_only(&self) -> bool;

        /// See [cef_process_message_t::copy]
        fn copy(&self) -> ProcessMessage {
            self.0.copy.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() {
                    None
                } else {
                    Some(ProcessMessage::from_raw(v))
                }
            })
        }

        /// See [cef_process_message_t::get_name]
        fn get_name(&self) -> CefString {
            self.0
                .get_name
                .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.0.get_this())) })
        }

        /// See [cef_process_message_t::get_argument_list]
        fn get_argument_list(&self) -> crate::ListValue {
            self.0.get_argument_list.and_then(|f| unsafe {
                let ptr = f(self.0.get_this());
                if ptr.is_null() {
                    None
                } else {
                    Some(crate::ListValue::from_raw(ptr))
                }
            })
        }
    );
    // TODO
    // See [cef_process_message_t::get_shared_memory_region]
    //fn get_shared_memory_region(&self) -> cef_sys::cef_shared_memory_region_t;
}
