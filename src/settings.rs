use std::ffi::c_int;

use cef_sys::{cef_color_t, cef_settings_t};

use crate::{string::CefString, CefLogItems, CefLogSeverity};

/// See [cef_settings_t] for more documentation.
#[derive(Debug, Default, Clone)]
pub struct CefSettings {
    pub no_sandbox: bool,
    pub browser_subprocess_path: Option<CefString>,
    pub framework_dir_path: Option<CefString>,
    pub main_bundle_path: Option<CefString>,
    pub multi_threaded_message_loop: bool,
    pub external_message_pump: bool,
    pub windowless_rendering_enabled: bool,
    pub command_line_args_disabled: bool,
    pub cache_path: Option<CefString>,
    pub root_cache_path: Option<CefString>,
    pub persist_session_cookies: bool,
    pub user_agent: Option<CefString>,
    pub user_agent_product: Option<CefString>,
    pub locale: Option<CefString>,
    pub log_file: Option<CefString>,
    pub log_severity: CefLogSeverity,
    pub log_items: CefLogItems,
    pub javascript_flags: Option<CefString>,
    pub resources_dir_path: Option<CefString>,
    pub locales_dir_path: Option<CefString>,
    pub remote_debugging_port: u32,
    pub uncaught_exception_stack_size: u32,
    pub background_color: u32,
    pub accept_language_list: Option<CefString>,
    pub cookieable_schemes_list: Option<CefString>,
    pub cookieable_schemes_exclude_defaults: bool,
    chrome_app_icon_id: i32,
    chrome_policy_id: Option<CefString>,
    #[cfg(target_family = "unix")]
    disable_signal_handlers: bool,
}

impl CefSettings {
    pub fn new() -> Self {
        Self {
            no_sandbox: true,
            remote_debugging_port: 5566,
            ..Default::default()
        }
    }

    pub fn as_raw(&self) -> cef_settings_t {
        cef_settings_t {
            size: std::mem::size_of::<cef_settings_t>(),
            no_sandbox: self.no_sandbox as c_int,
            browser_subprocess_path: self
                .browser_subprocess_path
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            framework_dir_path: self
                .framework_dir_path
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            main_bundle_path: self
                .main_bundle_path
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            chrome_app_icon_id: self.chrome_app_icon_id,
            chrome_policy_id: self
                .chrome_policy_id
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            #[cfg(target_family = "unix")]
            disable_signal_handlers: self.disable_signal_handlers as c_int,
            multi_threaded_message_loop: self.multi_threaded_message_loop as c_int,
            external_message_pump: self.external_message_pump as c_int,
            windowless_rendering_enabled: self.windowless_rendering_enabled as c_int,
            command_line_args_disabled: self.command_line_args_disabled as c_int,
            cache_path: self
                .cache_path
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            root_cache_path: self
                .root_cache_path
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            persist_session_cookies: self.persist_session_cookies as c_int,
            user_agent: self
                .user_agent
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            user_agent_product: self
                .user_agent_product
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            locale: self.locale.as_ref().map(|v| v.as_raw()).unwrap_or_default(),
            log_file: self
                .log_file
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            log_severity: self.log_severity,
            log_items: self.log_items,
            javascript_flags: self
                .javascript_flags
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            resources_dir_path: self
                .resources_dir_path
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            locales_dir_path: self
                .locales_dir_path
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            remote_debugging_port: self.remote_debugging_port as c_int,
            uncaught_exception_stack_size: self.uncaught_exception_stack_size as c_int,
            background_color: self.background_color as cef_color_t,
            accept_language_list: self
                .accept_language_list
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            cookieable_schemes_list: self
                .cookieable_schemes_list
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or_default(),
            cookieable_schemes_exclude_defaults: self.cookieable_schemes_exclude_defaults as c_int,
        }
    }

    pub fn from_raw(raw: cef_settings_t) -> Self {
        let cef_settings_t {
            size: _,
            no_sandbox,
            browser_subprocess_path,
            framework_dir_path,
            main_bundle_path,
            multi_threaded_message_loop,
            external_message_pump,
            windowless_rendering_enabled,
            command_line_args_disabled,
            cache_path,
            root_cache_path,
            persist_session_cookies,
            user_agent,
            user_agent_product,
            locale,
            log_file,
            log_severity,
            log_items,
            javascript_flags,
            resources_dir_path,
            locales_dir_path,
            remote_debugging_port,
            uncaught_exception_stack_size,
            background_color,
            accept_language_list,
            cookieable_schemes_list,
            cookieable_schemes_exclude_defaults,
            chrome_policy_id,
            chrome_app_icon_id,
            #[cfg(target_family = "unix")]
            disable_signal_handlers,
        } = raw;
        unsafe {
            Self {
                no_sandbox: no_sandbox == 1,
                browser_subprocess_path: CefString::from_raw(&browser_subprocess_path),
                framework_dir_path: CefString::from_raw(&framework_dir_path),
                main_bundle_path: CefString::from_raw(&main_bundle_path),
                multi_threaded_message_loop: multi_threaded_message_loop == 1,
                external_message_pump: external_message_pump == 1,
                windowless_rendering_enabled: windowless_rendering_enabled == 1,
                command_line_args_disabled: command_line_args_disabled == 1,
                cache_path: CefString::from_raw(&cache_path),
                root_cache_path: CefString::from_raw(&root_cache_path),
                persist_session_cookies: persist_session_cookies == 1,
                user_agent: CefString::from_raw(&user_agent),
                user_agent_product: CefString::from_raw(&user_agent_product),
                locale: CefString::from_raw(&locale),
                log_file: CefString::from_raw(&log_file),
                log_severity,
                log_items,
                javascript_flags: CefString::from_raw(&javascript_flags),
                resources_dir_path: CefString::from_raw(&resources_dir_path),
                locales_dir_path: CefString::from_raw(&locales_dir_path),
                remote_debugging_port: remote_debugging_port as _,
                uncaught_exception_stack_size: uncaught_exception_stack_size as _,
                background_color,
                accept_language_list: CefString::from_raw(&accept_language_list),
                cookieable_schemes_list: CefString::from_raw(&cookieable_schemes_list),
                cookieable_schemes_exclude_defaults: cookieable_schemes_exclude_defaults == 1,
                chrome_app_icon_id: chrome_app_icon_id as _,
                chrome_policy_id: CefString::from_raw(&chrome_policy_id),
                #[cfg(target_family = "unix")]
                disable_signal_handlers: disable_signal_handlers == 1,
            }
        }
    }
}
