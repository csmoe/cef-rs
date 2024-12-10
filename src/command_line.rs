use crate::prelude::*;

/// See [cef_command_line_t] for more documentation.
#[wrapper]
#[derive(Debug, Clone)]
pub struct CefCommandLine(cef_command_line_t);

impl CefCommandLine {
    wrapper_methods! {
        /// See [cef_command_line_t::get_command_line_string] for more documentation.
         fn get_command_line_string(&self) -> CefString {
            self.0.get_command_line_string.and_then(|f| unsafe { CefString::from_userfree_cef(f(self.0.get_this())) })
        }
    }
}
