use std::ffi::{c_char, CString};

use cef_sys::cef_main_args_t;

pub struct Args {
    args: std::env::Args,
    args_: Vec<*mut u8>,
}

impl std::fmt::Debug for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Args").field("args", &self.args).finish()
    }
}

impl Args {
    pub fn new(args: std::env::Args) -> Self {
        Self {
            args,
            args_: Vec::new(),
        }
    }
    pub(crate) fn as_raw(&mut self) -> cef_sys::cef_main_args_t {
        self.args_ = self
            .args
            .by_ref()
            .map(|mut arg| arg.as_mut_ptr())
            .collect::<Vec<_>>();
        cef_sys::cef_main_args_t {
            argc: self.args.len() as _,
            argv: self.args_.as_mut_ptr().cast(),
        }
    }
}
