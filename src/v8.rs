use crate::{prelude::*, rc::RcImpl};

/// See [cef_v8context_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefV8Context(cef_v8context_t);

impl CefV8Context {
    /// See [cef_v8context_get_current_context].
    pub fn get_current() -> Self {
        unsafe { Self::from_raw(cef_v8context_get_current_context()) }
    }

    /// See [cef_v8context_get_entered_context].
    pub fn get_entered() -> Self {
        unsafe { Self::from_raw(cef_v8context_get_entered_context()) }
    }

    /// See [cef_v8context_in_context].
    pub fn in_context() -> bool {
        unsafe { cef_v8context_in_context() == 1 }
    }

    wrapper_methods! {
        /// See [cef_v8context_t::is_valid].
        fn is_valid(&self) -> bool;

        /// See [cef_v8context_t::is_same].
        fn is_same(&self, other: Self) -> bool;

        /// See [cef_v8context_t::get_global].
        fn get_global(&self) -> V8Value {
            self.0.get_global.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() { None } else { Some(V8Value::from_raw(v)) }
            })
        }

        /// See [cef_v8context_t::get_browser].
        fn get_browser(&self) -> crate::browser::CefBrowser {
            self.0.get_browser.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() { None } else { Some(crate::browser::CefBrowser::from_raw(v)) }
            })
        }

        /// See [cef_v8context_t::get_frame].
        fn get_frame(&self) -> crate::CefFrame {
            self.0.get_frame.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() { None } else { Some(crate::CefFrame::from_raw(v)) }
            })
        }

        /// See [cef_v8context_t::enter].
        fn enter(&self) -> bool;

        /// See [cef_v8context_t::exit].
        fn exit(&self) -> bool;

        /// See [cef_v8context_t::eval].
        fn eval(&self,
            code: &str,
            script_url: &str,
            start_line: i32,
            retval: &mut V8Value,
            exception: &mut V8Excepction,
        ) -> bool {
            self.0.eval.map(|f| unsafe {
                let code = CefString::from(code);
                let script_url = CefString::from(script_url);
                f(self.0.get_this(), &code.as_raw(), &script_url.as_raw(), start_line as _,
                    &mut retval.clone().into_raw(), &mut exception.clone().into_raw() ) == 1
            })
        }

    }
}

/// See [cef_v8value_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct V8Value(cef_v8value_t);

impl V8Value {
    /// See [cef_v8value_create_undefined].
    pub fn undefined() -> Self {
        unsafe { Self::from_raw(cef_v8value_create_undefined()) }
    }

    /// See [cef_v8value_create_null].
    pub fn null() -> Self {
        unsafe { Self::from_raw(cef_v8value_create_null()) }
    }

    /// See [cef_v8value_create_bool].
    pub fn bool(value: bool) -> Self {
        unsafe { Self::from_raw(cef_v8value_create_bool(value as _)) }
    }

    /// See [cef_v8value_create_int].
    pub fn int(value: i32) -> Self {
        unsafe { Self::from_raw(cef_v8value_create_int(value as _)) }
    }

    /// See [cef_v8value_create_uint].
    pub fn uint(value: u32) -> Self {
        unsafe { Self::from_raw(cef_v8value_create_uint(value as _)) }
    }

    /// See [cef_v8value_create_double].
    pub fn double(value: f64) -> Self {
        unsafe { Self::from_raw(cef_v8value_create_double(value as _)) }
    }

    /// See [cef_v8value_create_string].
    pub fn string(value: &str) -> Self {
        unsafe { Self::from_raw(cef_v8value_create_string(&CefString::from(value).as_raw())) }
    }

    /// See [cef_v8value_create_array].
    pub fn array(length: usize) -> Self {
        unsafe { Self::from_raw(cef_v8value_create_array(length as _)) }
    }

    /// See [cef_v8value_create_object].
    pub fn object(accessor: V8Accessor, interceptor: V8Interceptor) -> Self {
        unsafe {
            Self::from_raw(cef_v8value_create_object(
                accessor.into_raw(),
                interceptor.into_raw(),
            ))
        }
    }

    /// See [cef_v8value_create_date].
    pub fn date(value: crate::CefBaseTime) -> Self {
        unsafe { Self::from_raw(cef_v8value_create_date(value)) }
    }

    /// See [cef_v8value_create_array_buffer].
    pub fn array_buffer(length: usize) -> Self {
        let mut buffer = vec![0; length];
        unsafe {
            let mut callback: cef_v8array_buffer_release_callback_t = std::mem::zeroed();
            callback.release_buffer = Some(release_buffer);
            let callback = RcImpl::new(callback, V8ArrayBufferReleaseCallback);

            extern "C" fn release_buffer(
                _: *mut _cef_v8array_buffer_release_callback_t,
                buffer: *mut core::ffi::c_void,
            ) {
                unsafe {
                    _ = Box::from_raw(buffer);
                }
            }

            let v = cef_v8value_create_array_buffer(
                buffer.as_mut_ptr().cast(),
                buffer.len() as _,
                callback.cast(),
            );
            core::mem::forget(buffer);
            Self::from_raw(v)
        }
    }

    /// See [cef_v8value_create_array_buffer_with_copy].
    pub fn array_buffer_with_copy(buffer: &mut [u8]) -> Self {
        let len = buffer.len();
        let ptr = buffer.as_mut_ptr();
        unsafe {
            Self::from_raw(cef_v8value_create_array_buffer_with_copy(
                ptr.cast(),
                len as _,
            ))
        }
    }

    /// See [cef_v8value_create_promise].
    pub fn promise() -> Self {
        unsafe { Self::from_raw(cef_v8value_create_promise()) }
    }

    /// See [cef_v8value_create_function].
    pub fn function(name: &str, handler: V8Handler) -> Self {
        unsafe {
            let name = CefString::from(name);
            Self::from_raw(cef_v8value_create_function(
                &name.as_raw(),
                handler.into_raw(),
            ))
        }
    }
}

impl V8Value {
    wrapper_methods! {
        /// See [cef_v8value_t::is_valid].
        fn is_valid(&self) -> bool;

        /// See [cef_v8value_t::is_undefined].
        fn is_undefined(&self) -> bool;

        /// See [cef_v8value_t::is_null].
        fn is_null(&self) -> bool;

        /// See [cef_v8value_t::is_bool].
        fn is_bool(&self) -> bool;

        /// See [cef_v8value_t::is_int].
        fn is_int(&self) -> bool;

        /// See [cef_v8value_t::is_uint].
        fn is_uint(&self) -> bool;

        /// See [cef_v8value_t::is_double].
        fn is_double(&self) -> bool;

        /// See [cef_v8value_t::is_string].
        fn is_string(&self) -> bool;

        /// See [cef_v8value_t::is_array].
        fn is_array(&self) -> bool;

        /// See [cef_v8value_t::is_object].
        fn is_object(&self) -> bool;

        /// See [cef_v8value_t::is_function].
        fn is_function(&self) -> bool;

        /// See [cef_v8value_t::is_date].
        fn is_date(&self) -> bool;

        /// See [cef_v8value_t::is_array_buffer].
        fn is_array_buffer(&self) -> bool;

        /// See [cef_v8value_t::is_promise].
        fn is_promise(&self) -> bool;

        /// See [cef_v8value_t::is_same].
        fn is_same(&self, other: Self) -> bool;

        /// See [cef_v8value_t::get_bool_value].
        fn get_bool_value(&self) -> bool {
            if !self.is_valid().unwrap_or_default() {
                return None;
            }
            self.0.get_bool_value.map(|f| unsafe {
                let v = f(self.0.get_this());
                v == 1
            })
        }

        /// See [cef_v8value_t::get_int_value].
        fn get_int_value(&self) -> i32 {
            if !self.is_valid().unwrap_or_default() {
                return None;
            }
            self.0.get_int_value.map(|f| unsafe { f(self.0.get_this()) })
        }

        /// See [cef_v8value_t::get_uint_value].
        fn get_uint_value(&self) -> u32 {
            if !self.is_valid().unwrap_or_default() {
                return None;
            }
            self.0.get_uint_value.map(|f| unsafe { f(self.0.get_this()) })
        }

        /// See [cef_v8value_t::get_double_value].
        fn get_double_value(&self) -> f64 {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.get_double_value.map(|f| unsafe { f(self.0.get_this()) })
        }

        /// See [cef_v8value_t::get_string_value].
        fn get_string_value(&self) -> CefString {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.get_string_value.and_then(|f| unsafe { CefString::from_userfree_cef(f(self.0.get_this()))})
        }

        /// See [cef_v8value_t::get_date_value].
        fn get_date_value(&self) -> crate::CefBaseTime {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.get_date_value.map(|f| unsafe { f(self.0.get_this())})
        }

        /// See [cef_v8value_t::is_user_created].
        fn is_user_created(&self) -> bool;

        /// See [cef_v8value_t::has_exception].
        fn has_exception(&self) -> bool;

        /// See [cef_v8value_t::get_exception].
        fn get_exception(&self) -> V8Excepction {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.get_exception.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() { None } else  {V8Excepction::from_raw(v).into()}
            })
        }

        /// See [cef_v8value_t::clear_exception].
        fn clear_exception(&self) {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.clear_exception.map(|f| unsafe { f(self.0.get_this()); })
        }

        /// See [cef_v8value_t::will_rethrow_exceptions].
        fn will_throw_exceptions(&self) -> bool {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.will_rethrow_exceptions.map(|f| unsafe { f(self.0.get_this()) == 1 })
        }

        /// See [cef_v8value_t::set_rethrow_exceptions].
        fn set_rethrow_exceptions(&self, rethrow: bool) {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.set_rethrow_exceptions.map(|f| unsafe { f(self.0.get_this(), rethrow as _); })
        }

        /// See [cef_v8value_t::has_value_bykey].
        fn has_value_bykey(&self, key: &str) -> bool {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.has_value_bykey.map(|f| unsafe { f(self.0.get_this(), &CefString::from(key).as_raw()) == 1 })
        }

        /// See [cef_v8value_t::get_value_bykey].
        fn get_value_bykey(&self, key: &str) -> Self {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.get_value_bykey.and_then(|f| unsafe {
                let v = f(self.0.get_this(), &CefString::from(key).as_raw());
                if v.is_null() { None } else { V8Value::from_raw(v).into() }
            })
        }

        /// See [cef_v8value_t::set_value_bykey].
        fn set_value_bykey(&self, key: &str, value: Self, attribute: crate::CefV8PropertyAttribute) {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.set_value_bykey.map(|f| unsafe {
                f(self.0.get_this(), &CefString::from(key).as_raw(), value.into_raw(), attribute as _);
            })
        }

        /// See [cef_v8value_t::get_value_byindex].
        fn get_value_byindex(&self, index: usize) -> Self {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.get_value_byindex.and_then(|f| unsafe {
                let v = f(self.0.get_this(), index as _);
                if v.is_null() { None } else { V8Value::from_raw(v).into() }
            })
        }

        /// See [cef_v8value_t::set_value_byindex].
        fn set_value_byindex(&self, index: usize, value: Self) {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.set_value_byindex.map(|f| unsafe { f(self.0.get_this(), index as _, value.into_raw()); })
        }

        /// See [cef_v8value_t::set_value_byaccessor].
        fn set_value_byaccessor(&self, key: &str,  attribute: crate::CefV8PropertyAttribute) {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.set_value_byaccessor.map(|f| unsafe { f(self.0.get_this(), &CefString::from(key).as_raw(),  attribute as _); })
        }

        /// See [cef_v8value_t::execute_function].
        fn execute_function(&self, object: V8Value, args: &[V8Value]) -> Self{
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.execute_function.map(|f| unsafe {
                let args = args.iter().map(|a| a.clone().into_raw()).collect::<Vec<_>>();
                let argc = args.len() as _;
                f(self.0.get_this(), object.into_raw(), argc, args.as_ptr()).into()
            })
        }

        /// See [cef_v8value_t::execute_function_with_context].
        fn execute_function_with_context(&self, context: CefV8Context, object: V8Value, args: &[V8Value]) -> Self{
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.execute_function_with_context.map(|f| unsafe {
                let args = args.iter().map(|a| a.clone().into_raw()).collect::<Vec<_>>();
                let argc = args.len() as _;
                f(self.0.get_this(), context.into_raw(), object.into_raw(), argc, args.as_ptr()).into()
            })
        }

        /// See [cef_v8value_t::resolve_promise].
        fn resolve_promise(&self, arg: V8Value) -> bool {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.resolve_promise.map(|f| unsafe { f(self.0.get_this(), arg.into_raw()) == 1 })
        }

        /// See [cef_v8value_t::reject_promise].
        fn reject_promise(&self, err: &str) -> bool {
            if !self.is_valid().unwrap_or_default() { return None; }
            self.0.reject_promise.map(|f| unsafe { f(self.0.get_this(), &CefString::from(err).as_raw()) == 1 })
        }

    }
}

/// See [cef_v8interceptor_t] for more documentation.
#[wrapper]
#[derive(Debug, Clone)]
pub struct V8Interceptor(cef_v8interceptor_t);

/// See [cef_v8exception_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct V8Excepction(cef_v8exception_t);

/// See [cef_v8array_buffer_release_callback_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
struct V8ArrayBufferReleaseCallback(cef_v8array_buffer_release_callback_t);

/// See [cef_v8stack_trace_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct V8StackTrace(cef_v8stack_trace_t);

/// See [cef_v8stack_frame_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct V8StackFrame(cef_v8stack_frame_t);

/// See [cef_v8accessor_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct V8Accessor(cef_v8accessor_t);

/// See [cef_v8handler_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct V8Handler(cef_v8handler_t);

impl V8Handler {
    wrapper_methods! {
        /// See [cef_v8handler_t::execute].
        fn execute(&self, name: &str, object: V8Value, args: &[V8Value], retval: V8Value, exception: &str) -> bool {
            let argv = args.iter().map(|a| unsafe {a.clone().into_raw()}).collect::<Vec<_>>();
            let argc = args.len();
            self.0.execute.map(|f| unsafe {
                f(self.0.get_this(), &CefString::from(name).as_raw(), object.into_raw(),
                    argc, argv.as_ptr(), &mut retval.into_raw(), &mut CefString::from(exception).as_raw()) == 1
            })
        }
    }
}
