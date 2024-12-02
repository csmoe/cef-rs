use crate::prelude::*;

/// See [cef_v8context_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct V8Context(cef_v8context_t);

impl V8Context {
    wrapper_methods! {
        /// See [cef_v8context_t::is_valid].
        fn is_valid(&self) -> bool;

        /// See [cef_v8context_t::get_global].
        fn get_global(&self) -> V8Value {
            self.0.get_global.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() { None } else { Some(V8Value::from_raw(v)) }
            })
        }

        /// See [cef_v8context_t::get_frame].
        fn get_frame(&self) -> crate::browser::frame::Frame {
            self.0.get_frame.and_then(|f| unsafe {
                let v = f(self.0.get_this());
                if v.is_null() { None } else { Some(crate::browser::frame::Frame::from_raw(v)) }
            })
        }
    }
}

/// See [cef_v8value_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct V8Value(cef_v8value_t);

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
pub struct V8ArrayBufferReleaseCallback(cef_v8array_buffer_release_callback_t);

/// See [cef_v8stack_trace_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct V8StackTrace(cef_v8stack_trace_t);

/// See [cef_v8stack_frame_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct V8StackFrame(cef_v8stack_frame_t);
