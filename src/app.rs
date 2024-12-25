use std::os::unix::ffi::OsStrExt;

use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    args::CefArgs, command_line::CefCommandLine, error::Error, error::Result, rc::RcImpl,
    settings::CefSettings, string::CefString,
};
use crate::{prelude::*, CefBrowser, CefDictionaryValue, CefFrame};

/// Handle process-specific callbacks
///
/// See [cef_app_t] for more documentation.
pub trait CefApp: Sized {
    type BrowserProcess: CefBrowserProcessHandler;
    type RenderProcess: CefRenderProcessHandler;

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

    fn get_browser_process_handler(&self) -> Option<Self::BrowserProcess> {
        None
    }

    fn get_render_process_handler(&self) -> Option<Self::RenderProcess> {
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
            let cmd = unsafe { CefCommandLine::from(command_line) };

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
                .map(|handler| handler.into_raw())
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
            handler.into_raw()
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
    settings: &CefSettings,
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

#[cfg(target_os = "macos")]
pub struct LibraryLoader {
    path: std::path::PathBuf,
}

impl LibraryLoader {
    const FRAMEWORK_PATH: &str =
        "Chromium Embedded Framework.framework/Chromium Embedded Framework";

    pub fn new(path: &std::path::Path, helper: bool) -> Self {
        let resolver = if helper { "../../.." } else { "../Frameworks" };
        let path = path.join(resolver).join(Self::FRAMEWORK_PATH);

        Self { path }
    }

    // See [cef_load_library] for more documentation.
    pub fn load(&self) -> Result<()> {
        Self::load_library(&self.path)
    }

    fn load_library(name: &std::path::Path) -> Result<()> {
        if unsafe { cef_load_library(name.as_os_str().as_bytes().as_ptr().cast()) } == 1 {
            Ok(())
        } else {
            Err(Error::CannotInit(0))
        }
    }
}

impl Drop for LibraryLoader {
    fn drop(&mut self) {
        unsafe {
            if cef_unload_library() != 1 {
                eprintln!("cannot unload framework {}", self.path.display());
            }
        }
    }
}

#[derive(Debug, Clone)]
#[wrapper]
/// See [cef_render_process_handler_t] for more documentation.
pub struct RenderProcessHandler(cef_sys::cef_render_process_handler_t);

#[allow(unused_variables)]
/// See [cef_render_process_handler_t] for more documentation.
pub trait CefRenderProcessHandler: Sized {
    /// See [cef_render_process_handler_t::on_web_kit_initialized] for more documentation.
    fn on_web_kit_initialized(&self) {}

    /// See [cef_render_process_handler_t::on_browser_created] for more documentation.
    fn on_browser_created(
        &self,
        browser: crate::CefBrowser,
        extra_info: crate::value::CefDictionaryValue,
    ) {
    }

    /// See [cef_render_process_handler_t::on_browser_destroyed] for more documentation.
    fn on_browser_destroyed(&self, browser: crate::CefBrowser) {}

    /// See [cef_render_process_handler_t::get_load_handler] for more documentation.
    fn get_load_handler(&self) -> Option<crate::handler::LoadHandler> {
        None
    }

    /// See [cef_render_process_handler_t::on_context_created] for more documentation.
    fn on_context_created(
        &self,
        browser: crate::CefBrowser,
        frame: crate::CefFrame,
        context: crate::v8::CefV8Context,
    ) {
    }

    /// See [cef_render_process_handler_t::on_context_released] for more documentation.
    fn on_context_released(
        &self,
        browser: crate::CefBrowser,
        frame: crate::CefFrame,
        context: crate::v8::CefV8Context,
    ) {
    }

    /// See [cef_render_process_handler_t::on_uncaught_exception] for more documentation.
    fn on_uncaught_exception(
        &self,
        browser: crate::CefBrowser,
        frame: crate::CefFrame,
        context: crate::v8::CefV8Context,
        exception: crate::v8::V8Excepction,
        stack_trace: crate::v8::V8StackTrace,
    ) {
    }

    // See [cef_render_process_handler_t::on_focused_node_changed] for more documentation.
    // fn on_focused_node_changed(&self, browser: *mut _cef_browser_t, frame: *mut _cef_frame_t, node: *mut _cef_domnode_t);

    /// See [cef_render_process_handler_t::on_process_message_received] for more documentation.
    fn on_process_message_received(
        &self,
        browser: crate::CefBrowser,
        frame: crate::CefFrame,
        source_process: crate::CefProcessId,
        message: crate::CefProcessMessage,
    ) -> bool {
        false
    }

    #[doc(hidden)]

    fn into_raw(self) -> *mut cef_render_process_handler_t {
        let mut handler: cef_render_process_handler_t = unsafe { std::mem::zeroed() };

        unsafe extern "C" fn on_web_kit_initialized<I: CefRenderProcessHandler>(
            self_: *mut _cef_render_process_handler_t,
        ) {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler.interface.on_web_kit_initialized();
        }

        unsafe extern "C" fn on_browser_created<I: CefRenderProcessHandler>(
            self_: *mut _cef_render_process_handler_t,
            browser: *mut _cef_browser_t,
            extra_info: *mut _cef_dictionary_value_t,
        ) {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler.interface.on_browser_created(
                CefBrowser::from(browser),
                CefDictionaryValue::from(extra_info),
            );
        }

        unsafe extern "C" fn on_browser_destroyed<I: CefRenderProcessHandler>(
            self_: *mut _cef_render_process_handler_t,
            browser: *mut _cef_browser_t,
        ) {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler
                .interface
                .on_browser_destroyed(CefBrowser::from(browser));
        }

        unsafe extern "C" fn get_load_handler<I: CefRenderProcessHandler>(
            self_: *mut _cef_render_process_handler_t,
        ) -> *mut _cef_load_handler_t {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler
                .interface
                .get_load_handler()
                .map(|h| h.into_raw())
                .unwrap_or(std::ptr::null_mut())
        }

        unsafe extern "C" fn on_context_created<I: CefRenderProcessHandler>(
            self_: *mut _cef_render_process_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            context: *mut _cef_v8context_t,
        ) {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler.interface.on_context_created(
                CefBrowser::from(browser),
                crate::CefFrame::from(frame),
                crate::v8::CefV8Context::from(context),
            );
        }

        unsafe extern "C" fn on_context_released<I: CefRenderProcessHandler>(
            self_: *mut _cef_render_process_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            context: *mut _cef_v8context_t,
        ) {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler.interface.on_context_released(
                CefBrowser::from(browser),
                crate::CefFrame::from(frame),
                crate::v8::CefV8Context::from(context),
            );
        }

        unsafe extern "C" fn on_uncaught_exception<I: CefRenderProcessHandler>(
            self_: *mut _cef_render_process_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            context: *mut _cef_v8context_t,
            exception: *mut _cef_v8exception_t,
            stack_trace: *mut _cef_v8stack_trace_t,
        ) {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler.interface.on_uncaught_exception(
                CefBrowser::from(browser),
                CefFrame::from(frame),
                crate::v8::CefV8Context::from(context),
                crate::v8::V8Excepction::from(exception),
                crate::v8::V8StackTrace::from(stack_trace),
            );
        }

        /*
                unsafe extern "C" fn on_focused_node_changed<I: CefRenderProcessHandler>(
                    self_: *mut _cef_render_process_handler_t,
                    browser: *mut _cef_browser_t,
                    frame: *mut _cef_frame_t,
                    node: *mut _cef_domnode_t,
                ) {
                    let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
                    handler.interface.on_focused_node_changed(
                        CefBrowser::from(browser),
                        CefFrame::from(frame),
                        node,
                    );
                }
        */

        unsafe extern "C" fn on_process_message_received<I: CefRenderProcessHandler>(
            self_: *mut _cef_render_process_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            source_process: cef_process_id_t,
            message: *mut _cef_process_message_t,
        ) -> ::std::os::raw::c_int {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler.interface.on_process_message_received(
                CefBrowser::from(browser),
                CefFrame::from(frame),
                source_process,
                crate::CefProcessMessage::from(message),
            ) as i32
        }

        handler.on_web_kit_initialized = Some(on_web_kit_initialized::<Self>);
        handler.on_browser_created = Some(on_browser_created::<Self>);
        handler.on_browser_destroyed = Some(on_browser_destroyed::<Self>);
        handler.get_load_handler = Some(get_load_handler::<Self>);
        handler.on_context_created = Some(on_context_created::<Self>);
        handler.on_context_released = Some(on_context_released::<Self>);
        handler.on_uncaught_exception = Some(on_uncaught_exception::<Self>);
        //handler.on_focused_node_changed = Some(on_focused_node_changed::<Self>);
        handler.on_process_message_received = Some(on_process_message_received::<Self>);

        crate::rc::RcImpl::new(handler, self).cast()
    }
}

impl CefRenderProcessHandler for () {
    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_render_process_handler_t {
        std::ptr::null_mut()
    }
}

#[derive(Debug, Clone)]
#[wrapper]
/// See [cef_browser_process_handler_t] for more documentation.
pub struct BrowserProcessHandler(cef_sys::cef_browser_process_handler_t);

#[allow(unused_variables)]
pub trait CefBrowserProcessHandler: Sized {
    /// See [cef_browser_process_handler_t::on_register_custom_preferences]
    fn on_register_custom_preferences(
        &self,
        type_: crate::CefPreferencesType,
        registrar: *mut _cef_preference_registrar_t,
    ) {
    }

    /// See [cef_browser_process_handler_t::on_context_initialized]
    fn on_context_initialized(&self) {}

    /// See [cef_browser_process_handler_t::on_before_child_process_launch]
    fn on_before_child_process_launch(&self, command_line: CefCommandLine) {}

    /// See [cef_browser_process_handler_t::on_already_running_app_relaunch]
    fn on_already_running_app_relaunch(
        &self,
        command_line: CefCommandLine,
        current_directory: Option<CefString>,
    ) -> bool {
        false
    }

    /// See [cef_browser_process_handler_t::on_schedule_message_pump_work]
    fn on_schedule_message_pump_work(&self, delay_ms: i64) {}

    /// See [cef_browser_process_handler_t::get_default_client]
    fn get_default_client(&self) -> *mut _cef_client_t {
        std::ptr::null_mut()
    }

    /// See [cef_browser_process_handler_t::get_default_request_context_handler]
    fn get_default_request_context_handler(&self) -> *mut _cef_request_context_handler_t {
        std::ptr::null_mut()
    }

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_browser_process_handler_t {
        let mut handler: cef_browser_process_handler_t = unsafe { std::mem::zeroed() };

        unsafe extern "C" fn on_register_custom_preferences<I: CefBrowserProcessHandler>(
            self_: *mut _cef_browser_process_handler_t,
            type_: cef_preferences_type_t,
            registrar: *mut _cef_preference_registrar_t,
        ) {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler
                .interface
                .on_register_custom_preferences(type_.into(), registrar);
        }

        unsafe extern "C" fn on_context_initialized<I: CefBrowserProcessHandler>(
            self_: *mut _cef_browser_process_handler_t,
        ) {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler.interface.on_context_initialized();
        }

        unsafe extern "C" fn on_before_child_process_launch<I: CefBrowserProcessHandler>(
            self_: *mut _cef_browser_process_handler_t,
            command_line: *mut _cef_command_line_t,
        ) {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);

            handler
                .interface
                .on_before_child_process_launch(CefCommandLine::from(command_line));
        }

        unsafe extern "C" fn on_already_running_app_relaunch<I: CefBrowserProcessHandler>(
            self_: *mut _cef_browser_process_handler_t,
            command_line: *mut _cef_command_line_t,
            current_directory: *const cef_string_t,
        ) -> ::std::os::raw::c_int {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler.interface.on_already_running_app_relaunch(
                CefCommandLine::from(command_line),
                CefString::from_raw(current_directory).into(),
            ) as i32
        }

        unsafe extern "C" fn on_schedule_message_pump_work<I: CefBrowserProcessHandler>(
            self_: *mut _cef_browser_process_handler_t,
            delay_ms: i64,
        ) {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler.interface.on_schedule_message_pump_work(delay_ms);
        }

        unsafe extern "C" fn get_default_client<I: CefBrowserProcessHandler>(
            self_: *mut _cef_browser_process_handler_t,
        ) -> *mut _cef_client_t {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler.interface.get_default_client()
        }

        unsafe extern "C" fn get_default_request_context_handler<I: CefBrowserProcessHandler>(
            self_: *mut _cef_browser_process_handler_t,
        ) -> *mut _cef_request_context_handler_t {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            handler.interface.get_default_request_context_handler()
        }

        handler.on_register_custom_preferences = Some(on_register_custom_preferences::<Self>);
        handler.on_context_initialized = Some(on_context_initialized::<Self>);
        handler.on_before_child_process_launch = Some(on_before_child_process_launch::<Self>);
        handler.on_already_running_app_relaunch = Some(on_already_running_app_relaunch::<Self>);
        handler.on_schedule_message_pump_work = Some(on_schedule_message_pump_work::<Self>);
        handler.get_default_client = Some(get_default_client::<Self>);
        handler.get_default_request_context_handler =
            Some(get_default_request_context_handler::<Self>);

        crate::rc::RcImpl::new(handler, self).cast()
    }
}

impl CefBrowserProcessHandler for () {
    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_browser_process_handler_t {
        std::ptr::null_mut()
    }
}

#[derive(Debug, Clone)]
#[wrapper]
/// See [cef_resource_bundle_handler_t] for more documentation.
pub struct ResourceBundleHandler(cef_sys::cef_resource_bundle_handler_t);

/// See [cef_resource_bundle_handler_t] for more documentation.
#[allow(unused_variables)]
pub trait CefResourceBundleHandler: Sized {
    /// See [cef_resource_bundle_handler_t::get_localized_string]
    fn get_localized_string(&self, string_id: i32, string: Option<CefString>) -> bool {
        false
    }

    /// See [cef_resource_bundle_handler_t::get_data_resource]
    fn get_data_resource(&self, resource_id: i32) -> Vec<u8> {
        vec![]
    }

    /// See [cef_resource_bundle_handler_t::get_data_resource_for_scale]
    fn get_data_resource_for_scale(
        &self,
        resource_id: i32,
        scale_factor: cef_scale_factor_t,
    ) -> Vec<u8> {
        vec![]
    }

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_resource_bundle_handler_t {
        let mut handler: cef_resource_bundle_handler_t = unsafe { std::mem::zeroed() };

        unsafe extern "C" fn get_localized_string<I: CefResourceBundleHandler>(
            self_: *mut _cef_resource_bundle_handler_t,
            string_id: ::std::os::raw::c_int,
            string: *mut cef_string_t,
        ) -> ::std::os::raw::c_int {
            let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let string = CefString::from_raw(string);
            let result = handler.interface.get_localized_string(string_id, string);
            result as ::std::os::raw::c_int
        }

        unsafe extern "C" fn get_data_resource<I: CefResourceBundleHandler>(
            self_: *mut _cef_resource_bundle_handler_t,
            resource_id: ::std::os::raw::c_int,
            data: *mut *mut ::std::os::raw::c_void,
            data_size: *mut usize,
        ) -> ::std::os::raw::c_int {
            /*
                        let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
                        let resource_data = handler.interface.get_data_resource(resource_id);
                        if !resource_data.is_empty() {
                            let boxed_data = Box::new(resource_data);
                            *data = Box::into_raw(boxed_data) as *mut ::std::os::raw::c_void;
                            *data_size = resource_data.len();
                            1
                        } else {
                            0
                        }
            */
            0
        }

        unsafe extern "C" fn get_data_resource_for_scale<I: CefResourceBundleHandler>(
            self_: *mut _cef_resource_bundle_handler_t,
            resource_id: ::std::os::raw::c_int,
            scale_factor: cef_scale_factor_t,
            data: *mut *mut ::std::os::raw::c_void,
            data_size: *mut usize,
        ) -> ::std::os::raw::c_int {
            /*
                        let handler: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
                        let resource_data = handler
                            .interface
                            .get_data_resource_for_scale(resource_id, scale_factor);
                        if !resource_data.is_empty() {
                            let boxed_data = Box::new(resource_data);
                            *data = Box::into_raw(boxed_data) as *mut ::std::os::raw::c_void;
                            *data_size = resource_data.len();
                            1
                        } else {
                            0
                        }
            */
            0
        }

        handler.get_localized_string = Some(get_localized_string::<Self>);
        handler.get_data_resource = Some(get_data_resource::<Self>);
        handler.get_data_resource_for_scale = Some(get_data_resource_for_scale::<Self>);

        crate::rc::RcImpl::new(handler, self).cast()
    }
}

impl CefResourceBundleHandler for () {
    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_resource_bundle_handler_t {
        std::ptr::null_mut()
    }
}
