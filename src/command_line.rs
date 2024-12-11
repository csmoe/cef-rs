use crate::prelude::*;

/// See [cef_command_line_t] for more documentation.
#[wrapper]
#[derive(Debug, Clone)]
pub struct CefCommandLine(cef_command_line_t);

impl CefCommandLine {
    wrapper_methods! {
        /// See [cef_command_line_t::init_from_argv]
        //fn init_from_argv(args: &crate::Args) -> Self {
        //    self.0.init_from_argv.and_then(|f| unsafe { CefCommandLine::from_raw(f(args.get_this())) })
        //}

        /// See [cef_command_line_t::get_command_line_string]
        fn get_command_line_string(&self) -> CefString {
            self.0.get_command_line_string.and_then(|f| unsafe { CefString::from_userfree_cef(f(self.0.get_this())) })
        }

        /// See [cef_command_line_t::append_switch]
        fn append_switch(&self, switch_name: &str) {
            unsafe {
                let switch_name = CefString::from(&switch_name);
                let Some(f) = self.0.append_switch else { return None };
                f(self.0.get_this(), &switch_name.as_raw());
                Some(())
            }
        }
    }
}
