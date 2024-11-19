use cef_sys::cef_render_handler_t;

use crate::{string::CefString, Browser, Rect};

crate::wrapper! {
    #[doc = "See [cef_render_handler_t] for more docs."]
    #[derive(Debug,Clone)]
    pub struct RenderHandler(cef_render_handler_t);
}

pub trait RenderCallback {
    fn get_accessibility_handler(&self) -> *mut cef_sys::cef_accessibility_handler_t;
    fn get_root_screen_rect(&self, browser: Browser, rect: Rect) -> ::std::os::raw::c_int;
    fn get_view_rect(&self, browser: Browser, rect: Rect);
    fn get_screen_point(
        &self,
        browser: Browser,
        view_x: i32,
        view_y: i32,
        screen_x: i32,
        screen_y: i32,
    ) -> ::std::os::raw::c_int;
    fn get_screen_info(
        &self,
        browser: Browser,
        screen_info: *mut cef_sys::cef_screen_info_t,
    ) -> ::std::os::raw::c_int;
    fn on_popup_show(&self, browser: Browser, show: bool);
    fn on_popup_size(&self, browser: Browser, rect: Rect);
    fn on_paint(
        &self,
        browser: Browser,
        type_: cef_sys::cef_paint_element_type_t,
        dirty_rects_count: usize,
        dirty_rects: Rect,
        buffer: *const ::std::os::raw::c_void,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
    fn on_accelerated_paint(
        &self,
        browser: Browser,
        type_: cef_sys::cef_paint_element_type_t,
        dirty_rects_count: usize,
        dirty_rects: Rect,
        info: *const cef_sys::cef_accelerated_paint_info_t,
    );
    fn get_touch_handle_size(
        &self,
        browser: Browser,
        orientation: cef_sys::cef_horizontal_alignment_t,
        size: crate::Size,
    );
    fn on_touch_handle_state_changed(
        &self,
        browser: Browser,
        state: *const cef_sys::cef_touch_handle_state_t,
    );
    fn start_dragging(
        &self,
        browser: Browser,
        drag_data: cef_sys::cef_drag_data_t,
        allowed_ops: cef_sys::cef_drag_operations_mask_t,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn update_drag_cursor(&self, browser: Browser, operation: cef_sys::cef_drag_operations_mask_t);
    fn on_scroll_offset_changed(&self, browser: Browser, x: f64, y: f64);
    fn on_ime_composition_range_changed(
        &self,
        browser: Browser,
        selected_range: cef_sys::cef_range_t,
        character_bounds_count: usize,
        character_bounds: Rect,
    );
    fn on_text_selection_changed(
        &self,
        browser: Browser,
        selected_text: CefString,
        selected_range: cef_sys::cef_range_t,
    );
    fn on_virtual_keyboard_requested(
        &self,
        browser: Browser,
        input_mode: cef_sys::cef_text_input_mode_t,
    );
}
