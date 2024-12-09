use crate::prelude::*;

/// See [cef_preference_manager_t] for more docs.
#[wrapper]
#[derive(Debug, Clone)]
pub struct Preferencemanager(cef_preference_manager_t);

impl Preferencemanager {
    /// See [cef_preference_manager_t::set_preference] for more docs.
    pub fn set_preference(&self, name: &str, value: crate::Value) -> Result<()> {
        let error = std::ptr::null_mut();
        let f = self.0.set_preference.ok_or(Error::NullPtr)?;
        unsafe {
            if f(
                self.0.get_this(),
                &CefString::from(name).as_raw(),
                value.into_raw(),
                error,
            ) == 1
            {
                Ok(())
            } else {
                Err(Error::Raw(CefString::from_raw(error)))
            }
        }
    }

    wrapper_methods! {
        /// See [cef_preference_manager_t::get_preference] for more docs.
        fn get_preference(self, name: &str) -> crate::Value {
            self.0.get_preference.and_then(|f| unsafe {
                let v = f(self.0.get_this(), &CefString::from(name).as_raw());
                if v.is_null() { None } else { Some(crate::Value::from_raw(v)) }
            })
        }
    }
}
