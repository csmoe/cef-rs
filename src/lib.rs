#![doc = include_str!("../README.md")]

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

pub use cef_sys as sys;

pub use app::*;
pub use browser::*;
pub use command_line::*;
pub use settings::*;
pub use view::*;

pub type LogSeverity = cef_sys::cef_log_severity_t;
pub type LogItems = cef_sys::cef_log_items_t;

/// The default value of `[Rect]` type is : { x: 0, y: 0, width: 1378, height: 800 }
pub type Rect = cef_sys::cef_rect_t;

pub type Size = cef_sys::cef_size_t;

pub type State = cef_sys::cef_state_t;

macro_rules! gen_fn {
    ($visibility:vis fn $method:ident(
        $($arg:ident: $t:ty)*)
    $(-> $($n:ident)?$value:path)?) => {
        $visibility fn $method(&self $(,$a: $t)*) $(-> $value)? {
            unsafe {
                let _result = self.0.$method.map(|f|
                    f(self.0.get_raw() $(,$crate::gen_fn!($c $arg))*)
                );

                $($crate::gen_fn!(return $($n)? _result))?
            }
        }
    };
    (into $arg:ident) => {
        $arg.0.into_raw()
    };
    (return $result:ident) => {
        $result
            .filter(|p| p.is_null())
            .map(|p| BrowserView(RefGuard::from_raw(p)))
    }
}
pub(crate) use gen_fn;

macro_rules! wrapper {
    (
    $(#[$attr:meta])*
    pub struct $name:ident($sys:path);
    $($visibility:vis fn $method:ident(
        &self
        $(,$arg:ident: [$ref:ident] $type:ty)*)
    $(->$value:path)?;)*
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

            $($crate::gen_fn!($visibility fn $method(
                $($arg: $ref $type)*
            )$(-> $value)?);)*
        }
    };
}
pub(crate) use wrapper;
