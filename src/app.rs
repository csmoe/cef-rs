use cef_sys::{
    cef_app_t, cef_command_line_t, cef_execute_process, cef_get_exit_code, cef_initialize,
    cef_quit_message_loop, cef_run_message_loop, cef_scheme_registrar_t, cef_shutdown,
    cef_string_t,
};

use crate::{
    args::Args, command_line::CommandLine, error::Error, error::Result, rc::RcImpl,
    settings::Settings, string::CefString,
};

/// Handle process-specific callbacks
///
/// See [cef_app_t] for more documentation.
pub trait App: Sized {
    fn on_before_command_line_processing(
        &self,
        _process_type: Option<CefString>,
        _command_line: CommandLine,
    ) {
    }

    fn on_register_custom_schemes(&self, _registrar: cef_scheme_registrar_t) {}

    fn get_resource_bundle_handler(&self) -> Option<cef_sys::cef_resource_bundle_handler_t> {
        None
    }

    fn get_browser_process_handler(&self) -> Option<cef_sys::cef_browser_process_handler_t> {
        None
    }

    fn get_render_process_handler(&self) -> Option<cef_sys::cef_render_process_handler_t> {
        None
    }

    fn into_raw(self) -> *mut cef_app_t {
        let mut object: cef_app_t = unsafe { std::mem::zeroed() };

        object.on_before_command_line_processing = Some(on_before_command_line_processing::<Self>);

        RcImpl::new(object, self).cast()
    }
}

/// See [cef_execute_process] for more documentation.
pub fn execute_process<T: App>(args: &mut Args, app: Option<T>) -> Result<()> {
    let args = args.as_raw()?;
    let app = app
        .map(|app| app.into_raw())
        .unwrap_or(std::ptr::null_mut());

    let code = unsafe { cef_execute_process(&args, app, std::ptr::null_mut()) };
    if code == -1 {
        Ok(())
    } else {
        if code == 0 {
            return Ok(());
        }

        Err(Error::Exit(code))
    }
}

/// See [cef_initialize] for more documentation.
pub fn initialize<T: App>(args: &mut Args, settings: &Settings, app: Option<T>) -> Result<()> {
    let args = args.as_raw()?;
    let settings = settings.as_raw();
    let app = app
        .map(|app| app.into_raw())
        .unwrap_or(std::ptr::null_mut());
    if unsafe { cef_initialize(&args, &settings, app, std::ptr::null_mut()) != 1 } {
        Err(Error::CannotInit(unsafe { cef_get_exit_code() }))
    } else {
        Ok(())
    }
}

/// See [cef_run_message_loop] for more documentation.
pub fn run_message_loop() {
    unsafe { cef_run_message_loop() }
}

/// See [cef_quit_message_loop] for more documentation.
pub fn quit_message_loop() {
    unsafe { cef_quit_message_loop() }
}

/// See [cef_shutdown] for more documentation.
pub fn shutdown() {
    unsafe { cef_shutdown() }
}

extern "C" fn on_before_command_line_processing<I: App>(
    this: *mut cef_app_t,
    process_type: *const cef_string_t,
    command_line: *mut cef_command_line_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let process_type = unsafe { CefString::from_raw(process_type) };
    let cmd = unsafe { CommandLine::from_raw(command_line) };

    obj.interface
        .on_before_command_line_processing(process_type, cmd);
}
