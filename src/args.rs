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
    pub(crate) fn as_raw(&mut self) -> crate::error::Result<cef_sys::cef_main_args_t> {
        self.args_ = self
            .args
            .by_ref()
            .map(|mut arg| arg.as_mut_ptr())
            .collect::<Vec<_>>();
        #[cfg(target_family = "unix")]
        return Ok(cef_sys::cef_main_args_t {
            argc: self.args.len() as _,
            argv: self.args_.as_mut_ptr().cast(),
        });

        #[cfg(target_family = "windows")]
        {
            use crate::error::Error;
            use windows::Win32::System::LibraryLoader::GetModuleHandleA;
            let instance = unsafe { GetModuleHandleA(None).map_err(Error::WinOs)? };
            Ok(cef_sys::cef_main_args_t {
                instance: instance.0.cast(),
            })
        }
    }
}
