use crate::add_view_delegate_methods;
use crate::prelude::*;
use crate::string::CefString;
use crate::CefRange;
use crate::CefTextFieldCommands;
use crate::CefTextStyle;
use crate::ViewDelegate;

/// See [cef_textfield_t] for more documentation.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefTextField(cef_textfield_t);

pub trait TextFieldDelegate: ViewDelegate {
    fn on_key_event(&self, _textfield: CefTextField, _event: cef_key_event_t) -> bool {
        false
    }
    fn on_after_user_action(&self, _textfield: CefTextField) {}

    fn into_raw(self) -> *mut cef_textfield_delegate_t {
        let mut object: cef_textfield_delegate_t = unsafe { std::mem::zeroed() };
        let view = &mut object.base;
        add_view_delegate_methods!(view);
        RcImpl::new(object, self).cast()
    }
}

impl CefTextField {
    pub fn create(delegate: impl TextFieldDelegate) -> Result<Self> {
        unsafe {
            let view = cef_sys::cef_textfield_create(<_ as TextFieldDelegate>::into_raw(delegate));
            if view.is_null() {
                return Err(Error::NullPtr);
            }
            Ok(Self::from(view))
        }
    }
}

impl CefTextField {
    wrapper_methods!(
        /// See [cef_textfield_t::set_password_input]
        fn set_password_input(&mut self, password_input: bool);
        /// See [cef_textfield_t::is_password_input]
        fn is_password_input(&self) -> bool;
        /// See [cef_textfield_t::set_read_only]
        fn set_read_only(&mut self, read_only: bool);
        /// See [cef_textfield_t::is_read_only]
        fn is_read_only(&self) -> bool;
        /// See [cef_textfield_t::get_text]
        fn get_text(&self) -> CefString {
            get_text.and_then(|f| unsafe { CefString::from_userfree_cef(f(self.get_this())) })
        }
        /// See [cef_textfield_t::set_text]
        fn set_text(&mut self, text: &CefString) {
            set_text.map(|f| unsafe { f(self.get_this(), &text.as_raw()) })
        }
        /// See [cef_textfield_t::append_text]
        fn append_text(&mut self, text: &CefString) {
            append_text.map(|f| unsafe { f(self.get_this(), &text.as_raw()) })
        }
        /// See [cef_textfield_t::insert_or_replace_text]
        fn insert_or_replace_text(&mut self, text: &CefString) {
            insert_or_replace_text.map(|f| unsafe { f(self.get_this(), &text.as_raw()) })
        }
        /// See [cef_textfield_t::has_selection]
        fn has_selection(&self) -> bool;
        /// See [cef_textfield_t::get_selected_text]
        fn get_selected_text(&self) -> CefString {
            get_selected_text
                .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.get_this())) })
        }
        /// See [cef_textfield_t::select_all]
        fn select_all(&mut self, reversed: bool);
        /// See [cef_textfield_t::clear_selection]
        fn clear_selection(&mut self);
        /// See [cef_textfield_t::get_selected_range]
        fn get_selected_range(&self) -> CefRange;
        /// See [cef_textfield_t::select_range]
        fn select_range(&mut self, range: CefRange) {
            select_range.map(|f| unsafe { f(self.get_this(), &range) })
        }
        /// See [cef_textfield_t::get_cursor_position]
        fn get_cursor_position(&self) -> usize;
        /// See [cef_textfield_t::set_text_color]
        fn set_text_color(&mut self, color: u32);
        /// See [cef_textfield_t::get_text_color]
        fn get_text_color(&self) -> u32;
        /// See [cef_textfield_t::set_selection_text_color]
        fn set_selection_text_color(&mut self, color: u32);
        /// See [cef_textfield_t::get_selection_text_color]
        fn get_selection_text_color(&self) -> u32;
        /// See [cef_textfield_t::set_selection_background_color]
        fn set_selection_background_color(&mut self, color: u32);
        /// See [cef_textfield_t::get_selection_background_color]
        fn get_selection_background_color(&self) -> u32;
        /// See [cef_textfield_t::set_font_list]
        fn set_font_list(&mut self, font_list: &CefString) {
            set_font_list.map(|f| unsafe { f(self.get_this(), &font_list.as_raw()) })
        }
        /// See [cef_textfield_t::apply_text_color]
        fn apply_text_color(&mut self, color: u32, range: CefRange) {
            apply_text_color.map(|f| unsafe { f(self.get_this(), color, &range) })
        }
        /// See [cef_textfield_t::apply_text_style]
        fn apply_text_style(&mut self, style: CefTextStyle, add: bool, range: CefRange) {
            apply_text_style.map(|f| unsafe { f(self.get_this(), style, add as i32, &range) })
        }
        /// See [cef_textfield_t::is_command_enabled]
        fn is_command_enabled(&self, command_id: CefTextFieldCommands) -> bool;
        /// See [cef_textfield_t::execute_command]
        fn execute_command(&mut self, command_id: CefTextFieldCommands);
        /// See [cef_textfield_t::clear_edit_history]
        fn clear_edit_history(&mut self);
        /// See [cef_textfield_t::set_placeholder_text]
        fn set_placeholder_text(&mut self, text: &CefString) {
            set_placeholder_text.map(|f| unsafe { f(self.get_this(), &text.as_raw()) })
        }
        /// See [cef_textfield_t::get_placeholder_text]
        fn get_placeholder_text(&self) -> CefString {
            get_placeholder_text
                .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.get_this())) })
        }
        /// See [cef_textfield_t::set_placeholder_text_color]
        fn set_placeholder_text_color(&mut self, color: u32);
        /// See [cef_textfield_t::set_accessible_name]
        fn set_accessible_name(&mut self, name: &CefString) {
            set_accessible_name.map(|f| unsafe { f(self.get_this(), &name.as_raw()) })
        }
    );
}
