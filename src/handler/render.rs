use crate::prelude::*;
use crate::{string::CefString, CefBrowser, CefRect};

/// See [cef_render_handler_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct RenderHandler(cef_render_handler_t);

/// See [cef_render_handler_t] for more docs.
#[allow(unused_variables)]
pub trait CefRenderHandler: Sized {
    /// See [cef_render_handler_t::get_accessibility_handler].
    fn get_accessibility_handler(&self) -> *mut cef_accessibility_handler_t {
        std::ptr::null_mut()
    }

    /// See [cef_render_handler_t::get_root_screen_rect].
    fn get_root_screen_rect(&self, browser: crate::CefBrowser, rect: &mut cef_rect_t) -> bool {
        false
    }

    /// See [cef_render_handler_t::get_view_rect].
    fn get_view_rect(&self, browser: crate::CefBrowser, rect: &mut cef_rect_t) {}

    /// See [cef_render_handler_t::get_screen_point].
    fn get_screen_point(
        &self,
        browser: crate::CefBrowser,
        view_x: i32,
        view_y: i32,
        screen_x: &mut i32,
        screen_y: &mut i32,
    ) -> bool {
        false
    }

    /// See [cef_render_handler_t::get_screen_info].
    fn get_screen_info(
        &self,
        browser: crate::CefBrowser,
        screen_info: &mut cef_screen_info_t,
    ) -> bool {
        false
    }

    /// See [cef_render_handler_t::on_popup_show].
    fn on_popup_show(&self, browser: crate::CefBrowser, show: bool) {}

    /// See [cef_render_handler_t::on_popup_size].
    fn on_popup_size(&self, browser: crate::CefBrowser, rect: &cef_rect_t) {}

    /// See [cef_render_handler_t::on_paint].
    fn on_paint(
        &self,
        browser: crate::CefBrowser,
        type_: cef_paint_element_type_t,
        dirty_rects: &[cef_rect_t],
        buffer: &[u8],
        width: i32,
        height: i32,
    ) {
    }

    /// See [cef_render_handler_t::on_accelerated_paint].
    fn on_accelerated_paint(
        &self,
        browser: crate::CefBrowser,
        type_: cef_paint_element_type_t,
        dirty_rects: &[cef_rect_t],
        info: &cef_accelerated_paint_info_t,
    ) {
    }

    /// See [cef_render_handler_t::get_touch_handle_size].
    fn get_touch_handle_size(
        &self,
        browser: crate::CefBrowser,
        orientation: cef_horizontal_alignment_t,
        size: &mut cef_size_t,
    ) {
    }

    /// See [cef_render_handler_t::on_touch_handle_state_changed].
    fn on_touch_handle_state_changed(
        &self,
        browser: crate::CefBrowser,
        state: &cef_touch_handle_state_t,
    ) {
    }

    /// See [cef_render_handler_t::start_dragging].
    fn start_dragging(
        &self,
        browser: crate::CefBrowser,
        drag_data: cef_drag_data_t,
        allowed_ops: cef_drag_operations_mask_t,
        x: i32,
        y: i32,
    ) -> bool {
        false
    }

    /// See [cef_render_handler_t::update_drag_cursor].
    fn update_drag_cursor(
        &self,
        browser: crate::CefBrowser,
        operation: cef_drag_operations_mask_t,
    ) {
    }

    /// See [cef_render_handler_t::on_scroll_offset_changed].
    fn on_scroll_offset_changed(&self, browser: crate::CefBrowser, x: f64, y: f64) {}

    /// See [cef_render_handler_t::on_ime_composition_range_changed].
    fn on_ime_composition_range_changed(
        &self,
        browser: crate::CefBrowser,
        selected_range: &cef_range_t,
        character_bounds: &[cef_rect_t],
    ) {
    }

    /// See [cef_render_handler_t::on_text_selection_changed].
    fn on_text_selection_changed(
        &self,
        browser: crate::CefBrowser,
        selected_text: CefString,
        selected_range: &cef_range_t,
    ) {
    }

    /// See [cef_render_handler_t::on_virtual_keyboard_requested].
    fn on_virtual_keyboard_requested(
        &self,
        browser: crate::CefBrowser,
        input_mode: cef_text_input_mode_t,
    ) {
    }

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_render_handler_t {
        unsafe extern "C" fn get_accessibility_handler<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
        ) -> *mut _cef_accessibility_handler_t {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            object.interface.get_accessibility_handler()
        }

        unsafe extern "C" fn get_root_screen_rect<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            rect: *mut cef_rect_t,
        ) -> ::std::os::raw::c_int {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            // FIXME
            let result = object.interface.get_root_screen_rect(browser, &mut *rect);
            result as i32
        }

        unsafe extern "C" fn get_view_rect<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            rect: *mut cef_rect_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object.interface.get_view_rect(browser, &mut *rect);
        }

        unsafe extern "C" fn get_screen_point<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            view_x: ::std::os::raw::c_int,
            view_y: ::std::os::raw::c_int,
            screen_x: *mut ::std::os::raw::c_int,
            screen_y: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            let result = object.interface.get_screen_point(
                browser,
                view_x,
                view_y,
                &mut *screen_x,
                &mut *screen_y,
            );
            result as i32
        }

        unsafe extern "C" fn get_screen_info<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            screen_info: *mut cef_screen_info_t,
        ) -> ::std::os::raw::c_int {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object.interface.get_screen_info(browser, &mut *screen_info) as i32
        }

        unsafe extern "C" fn on_popup_show<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            show: ::std::os::raw::c_int,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object.interface.on_popup_show(browser, show != 0);
        }

        unsafe extern "C" fn on_popup_size<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            rect: *const cef_rect_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object.interface.on_popup_size(browser, &*rect);
        }

        unsafe extern "C" fn on_paint<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            type_: cef_paint_element_type_t,
            dirty_rects_count: usize,
            dirty_rects: *const cef_rect_t,
            buffer: *const ::std::os::raw::c_void,
            width: ::std::os::raw::c_int,
            height: ::std::os::raw::c_int,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            let dirty_rects = std::slice::from_raw_parts(dirty_rects, dirty_rects_count);
            let buffer =
                std::slice::from_raw_parts(buffer as *const u8, (width * height * 4) as usize);
            object
                .interface
                .on_paint(browser, type_, dirty_rects, buffer, width, height);
        }

        unsafe extern "C" fn on_accelerated_paint<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            type_: cef_paint_element_type_t,
            dirty_rects_count: usize,
            dirty_rects: *const cef_rect_t,
            shared_handle: *const cef_accelerated_paint_info_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            let dirty_rects = std::slice::from_raw_parts(dirty_rects, dirty_rects_count);
            object
                .interface
                .on_accelerated_paint(browser, type_, dirty_rects, &*shared_handle);
        }

        unsafe extern "C" fn get_touch_handle_size<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            orientation: cef_horizontal_alignment_t,
            size: *mut cef_size_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object
                .interface
                .get_touch_handle_size(browser, orientation, &mut *size);
        }

        unsafe extern "C" fn on_touch_handle_state_changed<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            state: *const cef_touch_handle_state_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object
                .interface
                .on_touch_handle_state_changed(browser, &*state);
        }

        unsafe extern "C" fn start_dragging<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            drag_data: *mut _cef_drag_data_t,
            allowed_ops: cef_drag_operations_mask_t,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object
                .interface
                // FIXME
                .start_dragging(browser, *drag_data, allowed_ops, x, y) as i32
        }

        unsafe extern "C" fn update_drag_cursor<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            operation: cef_drag_operations_mask_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object.interface.update_drag_cursor(browser, operation);
        }

        unsafe extern "C" fn on_scroll_offset_changed<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            x: f64,
            y: f64,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object.interface.on_scroll_offset_changed(browser, x, y);
        }

        unsafe extern "C" fn on_ime_composition_range_changed<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            selected_range: *const cef_range_t,
            character_bounds_count: usize,
            character_bounds: *const cef_rect_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            let character_bounds =
                std::slice::from_raw_parts(character_bounds, character_bounds_count);
            object.interface.on_ime_composition_range_changed(
                browser,
                &*selected_range,
                character_bounds,
            );
        }

        unsafe extern "C" fn on_text_selection_changed<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            selected_text: *const cef_string_t,
            selected_range: *const cef_range_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            let selected_text = crate::CefString::from_raw(selected_text).unwrap_or_default();
            object
                .interface
                .on_text_selection_changed(browser, selected_text, &*selected_range);
        }

        unsafe extern "C" fn on_virtual_keyboard_requested<I: CefRenderHandler>(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            input_mode: cef_text_input_mode_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);
            let browser = crate::CefBrowser::from(browser);
            object
                .interface
                .on_virtual_keyboard_requested(browser, input_mode);
        }

        let mut object: cef_render_handler_t = unsafe { std::mem::zeroed() };
        object.get_accessibility_handler = Some(get_accessibility_handler::<Self>);
        object.get_root_screen_rect = Some(get_root_screen_rect::<Self>);
        object.get_view_rect = Some(get_view_rect::<Self>);
        object.get_screen_point = Some(get_screen_point::<Self>);
        object.get_screen_info = Some(get_screen_info::<Self>);
        object.on_popup_show = Some(on_popup_show::<Self>);
        object.on_popup_size = Some(on_popup_size::<Self>);
        object.on_paint = Some(on_paint::<Self>);
        object.on_accelerated_paint = Some(on_accelerated_paint::<Self>);
        object.get_touch_handle_size = Some(get_touch_handle_size::<Self>);
        object.on_touch_handle_state_changed = Some(on_touch_handle_state_changed::<Self>);
        object.start_dragging = Some(start_dragging::<Self>);
        object.update_drag_cursor = Some(update_drag_cursor::<Self>);
        object.on_scroll_offset_changed = Some(on_scroll_offset_changed::<Self>);
        object.on_ime_composition_range_changed = Some(on_ime_composition_range_changed::<Self>);
        object.on_text_selection_changed = Some(on_text_selection_changed::<Self>);
        object.on_virtual_keyboard_requested = Some(on_virtual_keyboard_requested::<Self>);

        crate::rc::RcImpl::new(object, self).cast()
    }
}

impl CefRenderHandler for () {
    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_render_handler_t {
        std::ptr::null_mut()
    }
}
