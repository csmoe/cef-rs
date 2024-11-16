use cef_sys::{
    cef_audio_handler_t, cef_browser_t, cef_client_t, cef_command_handler_t,
    cef_context_menu_handler_t, cef_dialog_handler_t, cef_display_handler_t,
    cef_download_handler_t, cef_drag_data_t, cef_drag_handler_t, cef_find_handler_t,
    cef_focus_handler_t, cef_frame_t, cef_jsdialog_handler_t, cef_keyboard_handler_t,
    cef_life_span_handler_t, cef_load_handler_t, cef_permission_handler_t, cef_print_handler_t,
    cef_process_id_t, cef_process_message_t, cef_render_handler_t, cef_request_handler_t,
};

use crate::rc::RcImpl;

/// See [cef_client_t] for more documentation.
pub trait Client: Sized {
    fn get_audio_handler(&self) -> Option<cef_audio_handler_t> {
        None
    }

    fn get_command_handler(&self) -> Option<cef_command_handler_t> {
        None
    }

    fn get_context_menu_handler(&self) -> Option<cef_context_menu_handler_t> {
        None
    }

    fn get_dialog_handler(&self) -> Option<cef_dialog_handler_t> {
        None
    }

    fn get_display_handler(&self) -> Option<cef_display_handler_t> {
        None
    }

    fn get_download_handler(&self) -> Option<cef_download_handler_t> {
        None
    }

    fn get_drag_handler(&self) -> Option<cef_drag_handler_t> {
        None
    }

    fn get_find_handler(&self) -> Option<cef_find_handler_t> {
        None
    }

    fn get_focus_handler(&self) -> Option<cef_focus_handler_t> {
        None
    }

    fn get_frame_handler(&self) -> Option<cef_frame_t> {
        None
    }

    fn get_permission_handler(&self) -> Option<cef_permission_handler_t> {
        None
    }

    fn get_jsdialog_handler(&self) -> Option<cef_jsdialog_handler_t> {
        None
    }

    fn get_keyboard_handler(&self) -> Option<cef_keyboard_handler_t> {
        None
    }

    fn get_life_span_handler(&self) -> Option<cef_life_span_handler_t> {
        None
    }

    fn get_load_handler(&self) -> Option<cef_load_handler_t> {
        None
    }

    fn get_print_handler(&self) -> Option<cef_print_handler_t> {
        None
    }

    fn get_request_handler(&self) -> Option<cef_request_handler_t> {
        None
    }

    fn get_render_handler(&self) -> Option<cef_render_handler_t> {
        None
    }

    fn on_process_message_received(
        &self,
        _browser: cef_browser_t,
        _frame: cef_frame_t,
        _source_process: cef_process_id_t,
        _message: cef_process_message_t,
    ) -> bool {
        false
    }

    fn into_raw(self) -> *mut cef_client_t {
        let object: cef_client_t = unsafe { std::mem::zeroed() };

        RcImpl::new(object, self).cast()
    }
}
