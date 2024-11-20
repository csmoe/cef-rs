#![doc = include_str!("../README.md")]

pub use cef_sys as sys;

mod app;
pub mod args;
mod browser;
pub mod client;
mod command_line;
mod error;
mod handler;
pub mod rc;
mod settings;
pub mod string;
mod view;
pub use app::*;
pub use browser::*;
pub use command_line::*;
pub use error::Error;
pub use error::Result;
use rc::Rc;
pub use settings::*;
pub use view::*;

mod alias {
    pub type LogSeverity = cef_sys::cef_log_severity_t;

    pub type LogItems = cef_sys::cef_log_items_t;

    /// The default value of `[Rect]` type is : { x: 0, y: 0, width: 1378, height: 800 }
    pub type Rect = cef_sys::cef_rect_t;

    pub type Size = cef_sys::cef_size_t;

    pub type State = cef_sys::cef_state_t;

    pub type GestureCommand = cef_sys::cef_gesture_command_t;
}
pub use alias::*;

pub trait IntoRawCallback: Rc {
    type RawDelegate;
    unsafe fn into_raw(&self) -> *mut Self::RawDelegate;
}

#[macro_export]
macro_rules! gen_fn {
    ($visibility:vis fn $method:ident(&self $(,$arg:ident: $type:ty)*)) => {
        $visibility fn $method(&self $(,$arg: $type)*) {
            unsafe {
                let _result = self.0.$method.map(|f|
                    f(self.0.get_raw() $(,$arg.into_raw())*)
                );
            }
        }
    };
    ($visibility:vis fn $method:ident(&self $(,$arg:ident: $type:ty)*) -> $ret:ty) => {
        $visibility fn $method(&self $(,$arg: $type)*) -> $ret {
            unsafe {
                self.0.$method.map(|f|
                    f(self.0.get_raw() $(,$arg.into_raw())*)
                )
            }
        }
    };
    ($visibility:vis fn $method:ident(&mut self $(,$arg:ident: $type:ty)*)) => {
        $visibility fn $method(&mut self $(,$arg: $type)*) {
            unsafe {
                let _result = self.0.$method.map(|f|
                    f(self.0.get_raw() $(,$arg.into_raw())*)
                );
            }
        }
    };
    ($visibility:vis fn $method:ident(&mut self $(,$arg:ident: $type:ty)*) -> $ret:ty) => {
        $visibility fn $method(&mut self $(,$arg: $type)*) -> $ret {
            unsafe {
                self.0.$method.map(|f|
                    f(self.0.get_raw() $(,$arg.into_raw())*)
                )
            }
        }
    };
}

macro_rules! wrapper {
    (
        $(#[$attr:meta])*
        pub struct $name:ident($sys:path);
        $(
            $visibility:vis fn $method:ident($(&$($mut:tt)?)? self $(, $arg:ident : $type:ty),* ) $(-> $ret:ty)?;
        )*
    ) => {
        $(#[$attr])*
        pub struct $name(pub(crate) $crate::rc::RefGuard<$sys>);

        impl $crate::rc::Rc for $sys {
            fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
                &self.base.as_base()
            }
        }

        impl $crate::rc::Rc for $name {
            fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
                self.0.as_base()
            }
        }

        impl $name {
            #[allow(clippy::missing_safety_doc)]
            pub unsafe fn from_raw(ptr: *mut $sys) -> Self {
                Self($crate::rc::RefGuard::from_raw(ptr))
            }

            #[allow(clippy::missing_safety_doc)]
            pub unsafe fn into_raw(self) -> *mut $sys {
                self.0.into_raw()
            }

            $(
                $crate::gen_fn!(
                    $visibility fn $method($(&$($mut)?)? self $(, $arg: $type)* ) $(-> $ret)?
                );
            )*
        }
    };
}

pub(crate) use wrapper;
