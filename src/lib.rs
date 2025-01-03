#![doc = include_str!("../README.md")]

mod app;
mod args;
mod browser;
mod client;
mod command_line;
mod error;
mod handler;
mod image;
mod interface;
mod menu_model;
mod multimap;
mod net;
mod preference_manager;
mod prelude;
mod process_message;
mod rc;
//mod sandbox;
mod scoped;
mod settings;
mod string;
mod task;
mod v8;
mod value;
mod view;

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU32, Ordering},
};

pub use app::*;
pub use args::*;
pub use browser::*;
pub use cef_sys as sys;
use cef_sys::cef_base_ref_counted_t;
pub use client::*;
pub use command_line::*;
pub use error::*;
pub use handler::*;
pub use image::*;
pub use menu_model::*;
pub use net::*;
pub use process_message::CefProcessMessage;
pub use settings::*;
pub use string::CefString;
pub use task::*;
pub use v8::*;
pub use value::*;
pub use view::*;

mod alias {
    pub type CefLogSeverity = cef_sys::cef_log_severity_t;

    pub type CefLogItems = cef_sys::cef_log_items_t;

    /// The default value of `[Rect]` type is : { x: 0, y: 0, width: 1378, height: 800 }
    pub type CefRect = cef_sys::cef_rect_t;

    pub type CefSize = cef_sys::cef_size_t;

    pub type CefState = cef_sys::cef_state_t;

    pub type CefGestureCommand = cef_sys::cef_gesture_command_t;

    pub type CefPoint = cef_sys::cef_point_t;

    pub type CefInsets = cef_sys::cef_insets_t;

    pub type CefTextStyle = cef_sys::cef_text_style_t;

    pub type CefTextFieldCommands = cef_sys::cef_text_field_commands_t;

    pub type CefRange = cef_sys::cef_range_t;

    pub type CefAxisAlignment = cef_sys::cef_axis_alignment_t;

    pub type CefMenuColorType = cef_sys::cef_menu_color_type_t;

    pub type CefProcessId = cef_sys::cef_process_id_t;

    pub type CefValueType = cef_sys::cef_value_type_t;

    pub type CefRuntimeStyle = cef_sys::cef_runtime_style_t;

    pub type CefChromeToolbarType = cef_sys::cef_chrome_toolbar_type_t;

    pub type CefBaseTime = cef_sys::cef_basetime_t;

    pub type CefV8PropertyAttribute = cef_sys::cef_v8_propertyattribute_t;

    pub type CefPostDataElementType = cef_sys::cef_postdataelement_type_t;

    pub type CefPreferencesType = cef_sys::cef_preferences_type_t;

    pub type CefPopupFeatures = cef_sys::cef_popup_features_t;

    pub type CefWindowOpenDisposition = cef_sys::cef_window_open_disposition_t;

    pub type CefCookieSameSite = cef_sys::cef_cookie_same_site_t;

    pub type CefCookiePriority = cef_sys::cef_cookie_priority_t;

    pub type CefThreadId = cef_sys::cef_thread_id_t;

    pub type CefErrorCode = cef_sys::cef_errorcode_t;

    pub type CefTransitionType = cef_sys::cef_transition_type_t;
}
pub use alias::*;

#[repr(C)]
pub(crate) struct RefCountWrapper<C, W> {
    __wrapper: W,
    count: AtomicU32,
    phantom: PhantomData<C>,
}
impl<C, W> Deref for RefCountWrapper<C, W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.__wrapper
    }
}
impl<C, W> DerefMut for RefCountWrapper<C, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.__wrapper
    }
}
impl<C, W> RefCountWrapper<C, W> {
    fn wrap_ptr<F>(wrapper: F) -> *mut C
    where
        F: FnOnce(cef_base_ref_counted_t) -> W,
    {
        let base = RefCountWrapper::<C, W> {
            __wrapper: wrapper(cef_base_ref_counted_t {
                size: std::mem::size_of::<Self>(),
                add_ref: Some(Self::add_ref),
                release: Some(Self::release),
                has_one_ref: Some(Self::has_one_ref),
                has_at_least_one_ref: Some(Self::has_at_least_one_ref),
            }),
            count: AtomicU32::new(1),
            phantom: PhantomData,
        };
        Box::into_raw(Box::new(base)) as *mut C
    }

    fn from_ptr<'a>(ptr: *mut cef_base_ref_counted_t) -> &'a mut RefCountWrapper<C, W> {
        unsafe { &mut *(ptr as *mut _) }
    }
    extern "C" fn add_ref(ptr: *mut cef_base_ref_counted_t) {
        let base = Self::from_ptr(ptr);
        base.count.fetch_add(1, Ordering::Relaxed);
    }
    extern "C" fn release(ptr: *mut cef_base_ref_counted_t) -> i32 {
        let base = Self::from_ptr(ptr);
        let old_count = base.count.fetch_sub(1, Ordering::Release);
        if old_count == 1 {
            unsafe { _ = Box::from_raw(base) };
            1
        } else {
            0
        }
    }
    extern "C" fn has_one_ref(ptr: *mut cef_base_ref_counted_t) -> i32 {
        let base = Self::from_ptr(ptr);
        if base.count.load(Ordering::SeqCst) == 1 {
            1
        } else {
            0
        }
    }
    extern "C" fn has_at_least_one_ref(ptr: *mut cef_base_ref_counted_t) -> i32 {
        let base = Self::from_ptr(ptr);
        if base.count.load(Ordering::SeqCst) >= 1 {
            1
        } else {
            0
        }
    }
}
