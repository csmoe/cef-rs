use cef_sys::cef_audio_handler_t;

crate::wrapper! {
    #[doc = "See [cef_audio_handler_t] for more docs."]
    #[derive(Debug,Clone)]
    pub struct AudioHandler(cef_audio_handler_t);
}

pub trait AudioCallback {
    //TODO
    //fn get_audio_parameters( browser: *mut _cef_browser_t, params: *mut cef_audio_parameters_t,) -> ::std::os::raw::c_int;
    //fn on_audio_stream_started( browser: *mut _cef_browser_t, params: *const cef_audio_parameters_t, channels: ::std::os::raw::c_int,);
    //fn on_audio_stream_packet( self_: *mut _cef_audio_handler_t, browser: *mut _cef_browser_t, data: *mut *const f32, frames: ::std::os::raw::c_int, pts: i64,);
    //fn on_audio_stream_stopped(self_: *mut _cef_audio_handler_t, browser: *mut _cef_browser_t);
    //fn on_audio_stream_error( self_: *mut _cef_audio_handler_t, browser: *mut _cef_browser_t, message: *const cef_string_t,);
}
