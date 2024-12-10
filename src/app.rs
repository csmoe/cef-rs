use crate::prelude::*;
use crate::{
    args::CefArgs, command_line::CefCommandLine, error::Error, error::Result, rc::RcImpl,
    settings::Settings, string::CefString,
};

/// Handle process-specific callbacks
///
/// See [cef_app_t] for more documentation.
pub trait CefApp: Sized {
    fn on_before_command_line_processing(
        &self,
        _process_type: Option<CefString>,
        _command_line: CefCommandLine,
    ) {
    }

    fn on_register_custom_schemes(&self, _registrar: cef_scheme_registrar_t) {}

    fn get_resource_bundle_handler(&self) -> Option<RenderProcessHandler> {
        None
    }

    fn get_browser_process_handler(&self) -> Option<BrowserProcessHandler> {
        None
    }

    fn get_render_process_handler(&self) -> Option<RenderProcessHandler> {
        None
    }

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_app_t {
        let mut object: cef_app_t = unsafe { std::mem::zeroed() };

        extern "C" fn on_before_command_line_processing<I: CefApp>(
            this: *mut cef_app_t,
            process_type: *const cef_string_t,
            command_line: *mut cef_command_line_t,
        ) {
            let obj: &mut RcImpl<_, I> = RcImpl::get(this);
            let process_type = unsafe { CefString::from_raw(process_type) };
            let cmd = unsafe { CefCommandLine::from_raw(command_line) };

            obj.interface
                .on_before_command_line_processing(process_type, cmd);
        }
        object.on_before_command_line_processing = Some(on_before_command_line_processing::<Self>);

        extern "C" fn get_render_process_handler<I: CefApp>(
            this: *mut cef_app_t,
        ) -> *mut cef_render_process_handler_t {
            let app: &mut RcImpl<_, I> = RcImpl::get(this);

            app.interface
                .get_render_process_handler()
                .map(|handler| unsafe { handler.into_raw() })
                .unwrap_or(std::ptr::null_mut())
        }
        object.get_render_process_handler = Some(get_render_process_handler::<Self>);

        extern "C" fn get_browser_process_handler<I: CefApp>(
            this: *mut cef_app_t,
        ) -> *mut cef_browser_process_handler_t {
            let app: &mut RcImpl<_, I> = RcImpl::get(this);
            let Some(handler) = app.interface.get_browser_process_handler() else {
                return std::ptr::null_mut();
            };
            unsafe { handler.into_raw() }
        }
        object.get_browser_process_handler = Some(get_browser_process_handler::<Self>);

        RcImpl::new(object, self).cast()
    }
}

/// See [cef_execute_process] for more documentation.
pub fn execute_process<T: CefApp>(args: &mut CefArgs, app: Option<T>) -> Result<()> {
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
pub fn initialize<T: CefApp>(
    args: &mut CefArgs,
    settings: &Settings,
    app: Option<T>,
) -> Result<()> {
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

/// See [cef_do_message_loop_work] for more documentation.
pub fn do_message_loop_work() {
    unsafe { cef_do_message_loop_work() }
}

#[derive(Debug, Clone)]
#[wrapper]
/// See [cef_render_process_handler_t] for more documentation.
pub struct RenderProcessHandler(cef_sys::cef_render_process_handler_t);

impl RenderProcessHandler {
    wrapper_methods! {
        /// See [cef_render_process_handler_t::on_web_kit_initialized] for more documentation.
        fn on_web_kit_initialized(&self);

        /// See [cef_render_process_handler_t::on_browser_created] for more documentation.
        fn on_browser_created(&self, browser: crate::CefBrowser, extra_info: crate::value::CefDictionaryValue);

        /// See [cef_render_process_handler_t::on_browser_destroyed] for more documentation.
        fn on_browser_destroyed(&self, browser: crate::CefBrowser);

        /// See [cef_render_process_handler_t::get_load_handler] for more documentation.
        fn get_load_handler(&self) -> crate::handler::LoadHandler {
            self.0.get_load_handler.and_then(|f| unsafe {
                let h = f(self.0.get_this());
                if h.is_null() { return None; }
                crate::handler::LoadHandler::from_raw(h).into()
            })
        }

        /// See [cef_render_process_handler_t::on_context_created] for more documentation.
        fn on_context_created(&self, browser: crate::CefBrowser, frame: crate::frame::CefFrame, context: crate::v8::CefV8Context);

        fn on_context_released(&self, browser: crate::CefBrowser, frame: crate::frame::CefFrame, context: crate::v8::CefV8Context) ;

        fn on_uncaught_exception(&self, browser: crate::CefBrowser, frame: crate::frame::CefFrame, context: crate::v8::CefV8Context, exception: crate::v8::V8Excepction, stack_trace: crate::v8::V8StackTrace);

        // fn on_focused_node_changed(&self, browser: *mut _cef_browser_t, frame: *mut _cef_frame_t, node: *mut _cef_domnode_t);

        fn on_process_message_received(&self, browser: crate::CefBrowser, frame: crate::frame::CefFrame, source_process: crate::CefProcessId, message: crate::ProcessMessage) -> bool;
    }
}

#[derive(Debug, Clone)]
#[wrapper]
/// See [cef_browser_process_handler_t] for more documentation.
pub struct BrowserProcessHandler(cef_sys::cef_browser_process_handler_t);

impl BrowserProcessHandler {
    wrapper_methods! {
        /// See [cef_browser_process_handler_t::on_register_custom_preferences]
        fn on_register_custom_preferences(&self, type_: crate::CefPreferencesType, registrar: *mut _cef_preference_registrar_t);

        /// See [cef_browser_process_handler_t::on_context_initialized]
        fn on_context_initialized(&self);

        /// See [cef_browser_process_handler_t::on_before_child_process_launch]
        fn on_before_child_process_launch(&self, command_line: *mut _cef_command_line_t);

        /// See [cef_browser_process_handler_t::on_already_running_app_relaunch]
        fn on_already_running_app_relaunch(&self, command_line: *mut _cef_command_line_t, current_directory: *const cef_string_t) -> bool;

        /// See [cef_browser_process_handler_t::on_schedule_message_pump_work]
        fn on_schedule_message_pump_work(&self, delay_ms: i64);

        /// See [cef_browser_process_handler_t::get_default_client]
        fn get_default_client(&self) -> *mut _cef_client_t;

        /// See [cef_browser_process_handler_t::get_default_request_context_handler]
        fn get_default_request_context_handler(&self) -> *mut _cef_request_context_handler_t;
    }
}

#[derive(Debug, Clone)]
#[wrapper]
/// See [cef_resource_bundle_handler_t] for more documentation.
pub struct ResourceBundleHandler(cef_sys::cef_resource_bundle_handler_t);

impl ResourceBundleHandler {
    wrapper_methods! {
        /// See [cef_resource_bundle_handler_t::get_localized_string]
        fn get_localized_string(&self, string_id: i32, string: &str) -> bool {
            self.0.get_localized_string.map(|f| unsafe {
                f(self.0.get_this(), string_id as _, &mut CefString::from(string).as_raw()) == 1
            })
        }

        /// See [cef_resource_bundle_handler_t::get_data_resource]
        fn get_data_resource(&self, resource_id: i32) -> Vec<u8> {
            self.0.get_data_resource.and_then(|f| unsafe {
                let data: *mut u8 = std::ptr::null_mut();
                let mut size = 0;
                if f(self.0.get_this(), resource_id as _, data.cast(), &mut size) == 1 {
                    std::slice::from_raw_parts(data, size).to_vec().into()
                } else {
                    None
                }
            })
        }

        /// See [cef_resource_bundle_handler_t::get_data_resource_for_scale]
        fn get_data_resource_for_scale(&self, resource_id: i32, scale_factor: cef_scale_factor_t) -> Vec<u8> {
            self.0.get_data_resource_for_scale.and_then(|f| unsafe {
                let data: *mut u8 = std::ptr::null_mut();
                let mut size = 0;
                if f(self.0.get_this(), resource_id as _, scale_factor, data.cast(), &mut size) == 1 {
                    std::slice::from_raw_parts(data, size).to_vec().into()
                } else {
                    None
                }
            })
        }
    }
}
