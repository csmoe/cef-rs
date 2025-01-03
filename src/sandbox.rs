use std::os::raw::c_void;

use cef_sys::cef_sandbox_info_create;
use cef_sys::cef_sandbox_info_destroy;

pub struct CefSandbox(*mut c_void);

impl CefSandbox {
    /// See [cef_sandbox_info_create]
    #[cfg(target_os = "windows")]
    pub fn init() -> Self {
        Self(unsafe { cef_sandbox_info_create() })
    }

    /// See [cef_sandbox_initialize]
    #[cfg(target_os = "macos")]
    pub fn init(args: std::env::Args) -> Self {
        Self(unsafe { cef_sandbox_initialize() })
    }
}

impl Drop for CefSandbox {
    fn drop(&mut self) {
        if self.0.is_null() {
            return;
        }
        unsafe {
            #[cfg(target_os = "windows")]
            {
                return cef_sandbox_info_destroy(self.0);
            }
            #[cfg(target_os = "macos")]
            {
                return cef_sandbox_destroy(self.0);
            }
        }
    }
}
