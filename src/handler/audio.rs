use crate::prelude::*;
use crate::string::CefString;
use crate::Browser;

/// See [cef_audio_handler_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct AudioHandler(cef_audio_handler_t);

//TODO
pub trait AudioCallback {
    /// See [cef_audio_handler_t::get_audio_parameters]
    fn get_audio_parameters(
        &self,
        _browser: Browser,
        _params: *mut cef_audio_parameters_t,
    ) -> bool {
        todo!()
    }

    /// See [cef_audio_handler_t::on_audio_stream_started]
    fn on_audio_stream_started(
        &self,
        _browser: *mut _cef_browser_t,
        _params: *const cef_audio_parameters_t,
        _channels: ::std::os::raw::c_int,
    ) {
    }

    /// See [cef_audio_handler_t::on_audio_stream_packet]
    fn on_audio_stream_packet(
        &self,
        _browser: Browser,
        _data: *mut *const f32,
        _frames: ::std::os::raw::c_int,
        _pts: i64,
    ) {
    }

    /// See [cef_audio_handler_t::on_audio_stream_stopped]
    fn on_audio_stream_stopped(&self, _browser: Browser) {}

    /// See [cef_audio_handler_t::on_audio_stream_error]
    fn on_audio_stream_error(&self, _browser: Browser, _message: CefString) {}
}
