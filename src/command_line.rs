use std::str::FromStr;

use crate::{
    interface::{Base, CefInterface},
    prelude::*,
};

/// See [cef_command_line_t] for more documentation.
#[wrapper]
#[derive(Debug, Clone)]
pub struct CefCommandLine(cef_command_line_t);

#[derive(Debug)]
pub enum CefProcessType {
    Uknown,
    Browser,
    GPU,
    Renderer,
    Utility,
    #[cfg(target_os = "linux")]
    Zygote,
}

impl std::fmt::Display for CefProcessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CefProcessType::*;
        match self {
            Browser => write!(f, "browser"),
            Renderer => write!(f, "renderer"),
            Utility => write!(f, "utility"),
            #[cfg(target_os = "linux")]
            Zygote => write!(f, "zygote"),
            GPU => write!(f, "gpu-process"),
            Uknown => write!(f, "unknown"),
        }
    }
}

impl FromStr for CefProcessType {
    type Err = &'static str;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use CefProcessType::*;
        match s {
            "browser" => Ok(Browser),
            "renderer" => Ok(Renderer),
            "utility" => Ok(Utility),
            #[cfg(target_os = "linux")]
            "zygote" => Ok(Zygote),
            "gpu-process" => Ok(GPU),
            _ => Ok(Uknown),
        }
    }
}

impl CefCommandLine {
    /// See [cef_command_line_create]
    pub fn create() -> Result<Self> {
        let ptr = unsafe { cef_command_line_create() };
        if ptr.is_null() {
            return Err(Error::NullPtr);
        }
        Ok(CefCommandLine::from(ptr))
    }

    pub fn process_type(&self) -> CefProcessType {
        const TYPE: &str = "type";
        if !self.has_switch(TYPE).unwrap_or_default() {
            return CefProcessType::Browser;
        }
        self.get_switch_value(TYPE)
            .unwrap_or_default()
            .to_string()
            .parse()
            .unwrap()
    }

    wrapper_methods! {
        /// See [cef_command_line_t::init_from_argv]
        #[cfg(not(target_os = "windows"))]
        fn init_from_argv(&mut self, args: &mut crate::CefArgs) {
            let Ok(args) = args.as_raw() else { return None; };
             init_from_argv.map(|f| unsafe { f(self.get_this(), args.argc, args.argv.cast()) })
        }

        /// See [cef_command_line_t::init_from_string]
        #[cfg(target_os = "windows")]
        fn init_from_string(&mut self, args: &str) {
            let args = CefString::from(args);
             init_from_string.map(|f| unsafe { f(self.get_this(), &args.as_raw()) })
        }

        /// See [cef_command_line_t::get_command_line_string]
        fn get_command_line_string(&self) -> CefString {
             get_command_line_string.and_then(|f| unsafe { CefString::from_userfree_cef(f(self.get_this())) })
        }

        /// See [cef_command_line_t::append_switch]
        fn append_switch(&self, switch_name: &str) {
            unsafe {
                let switch_name = CefString::from(&switch_name);
                let Some(f) =  append_switch else { return None };
                f(self.get_this(), &switch_name.as_raw());
                Some(())
            }
        }

        /// See [cef_command_line_t::append_switch_with_value]
        fn append_switch_with_value(&self, name: &str, value: &str) {
            unsafe {
                append_switch_with_value.map(|f|{f(self.get_this(), &CefString::from(name).as_raw(), &CefString::from(value).as_raw())})
            }
        }

        /// See [cef_command_line_t::get_switch_value]
        fn get_switch_value(&self, switch_name: &str) -> CefString {
             get_switch_value.and_then(|f| unsafe {
                CefString::from_userfree_cef(f(self.get_this(), &CefString::from(&switch_name).as_raw()))
            })
        }

        /// See [cef_command_line_t::has_switch]
        fn has_switch(&self, switch_name: &str) -> bool {
            unsafe {
                let switch_name = CefString::from(&switch_name);
                let Some(f) =  has_switch else { return false.into() };
                let has = f(self.get_this(), &switch_name.as_raw()) == 1;
                has.into()
            }
        }
    }
}
