use crate::prelude::*;

/// See [cef_preference_manager_t] for more docs.
#[wrapper]
#[derive(Debug, Clone)]
pub struct Preferencemanager(cef_preference_manager_t);

impl Preferencemanager {
    wrapper_methods! {
        /// See [cef_preference_manager_t::get_preference] for more docs.
        fn get_preference(self, name: &str) -> crate::CefValue {
             get_preference.map(|f| unsafe {
                let v = f(self.get_this(), &CefString::from(name).as_raw());
                crate::CefValue::from(v)
            })
        }


        /// See [cef_preference_manager_t::set_preference] for more docs.
        fn set_preference(&self, name: &str, value: crate::CefValue) {
            let error = std::ptr::null_mut();
            let f = set_preference?;
            // FIXME
            unsafe {
                if f(
                    self.get_this(),
                    &CefString::from(name).as_raw(),
                    value.into_raw(),
                    error,
                ) == 1
                {
                    Some(())
                } else {
                    None
                }
            }
        }
    }
}
