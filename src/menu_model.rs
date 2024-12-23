use crate::prelude::*;

/// See [cef_menu_model_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefMenuModel(cef_menu_model_t);

impl CefMenuModel {
    /// See [cef_menu_model_create] for more docs.
    pub fn create(delegate: impl MenuModelDelegate) -> Result<Self> {
        unsafe {
            let m = cef_menu_model_create(delegate.into_raw());
            if m.is_null() {
                Err(Error::NullPtr)
            } else {
                Ok(Self::from(m))
            }
        }
    }
}

impl CefMenuModel {
    wrapper_methods!(
        /// See [cef_menu_model_t::is_sub_menu]
        fn is_sub_menu(&self) -> bool;

        /// See [cef_menu_model_t::clear]
        fn clear(&mut self) -> bool;

        /// See [cef_menu_model_t::get_count]
        fn get_count(&self) -> usize;

        /// See [cef_menu_model_t::add_separator]
        fn add_separator(&mut self) -> bool;

        /// See [cef_menu_model_t::add_item]
        fn add_item(&mut self, command_id: i32, label: CefString) -> bool {
            add_item.map(|f| unsafe { f(self.get_this(), command_id, &label.as_raw()) == 1 })
        }

        /// See [cef_menu_model_t::add_check_item]
        fn add_check_item(&mut self, command_id: i32, label: CefString) -> bool {
            add_check_item.map(|f| unsafe { f(self.get_this(), command_id, &label.as_raw()) == 1 })
        }

        /// See [cef_menu_model_t::add_radio_item]
        fn add_radio_item(&mut self, command_id: i32, label: CefString, group_id: i32) -> bool {
            add_radio_item
                .map(|f| unsafe { f(self.get_this(), command_id, &label.as_raw(), group_id) == 1 })
        }

        /// See [cef_menu_model_t::add_sub_menu]
        fn add_sub_menu(&mut self, command_id: i32, label: CefString) -> CefMenuModel {
            add_sub_menu.and_then(|f| unsafe {
                let m = f(self.get_this(), command_id, &label.as_raw());
                if m.is_null() {
                    None
                } else {
                    CefMenuModel::from(m).into()
                }
            })
        }

        /// See [cef_menu_model_t::insert_separator_at]
        fn insert_separator_at(&mut self, index: usize) -> bool;

        /// See [cef_menu_model_t::insert_item_at]
        fn insert_item_at(&mut self, index: usize, command_id: i32, label: CefString) -> bool {
            insert_item_at
                .map(|f| unsafe { f(self.get_this(), index, command_id, &label.as_raw()) == 1 })
        }

        /// See [cef_menu_model_t::insert_check_item_at]
        fn insert_check_item_at(
            &mut self,
            index: usize,
            command_id: i32,
            label: CefString,
        ) -> bool {
            insert_check_item_at
                .map(|f| unsafe { f(self.get_this(), index, command_id, &label.as_raw()) == 1 })
        }

        /// See [cef_menu_model_t::insert_radio_item_at]
        fn insert_radio_item_at(
            &mut self,
            index: usize,
            command_id: i32,
            label: CefString,
            group_id: i32,
        ) -> bool {
            insert_radio_item_at.map(|f| unsafe {
                f(
                    self.get_this(),
                    index,
                    command_id,
                    &label.as_raw(),
                    group_id,
                ) == 1
            })
        }

        /// See [cef_menu_model_t::insert_sub_menu_at]
        fn insert_sub_menu_at(
            &mut self,
            index: usize,
            command_id: i32,
            label: CefString,
        ) -> CefMenuModel {
            insert_sub_menu_at.and_then(|f| unsafe {
                let m = f(self.get_this(), index, command_id, &label.as_raw());
                if m.is_null() {
                    None
                } else {
                    CefMenuModel::from(m).into()
                }
            })
        }

        /// See [cef_menu_model_t::remove]
        fn remove(&mut self, command_id: i32) -> bool;

        /// See [cef_menu_model_t::remove_at]
        fn remove_at(&mut self, index: usize) -> bool;

        /// See [cef_menu_model_t::get_index_of]
        fn get_index_of(&self, command_id: i32) -> i32;

        /// See [cef_menu_model_t::get_command_id_at]
        fn get_command_id_at(&self, index: usize) -> i32;

        /// See [cef_menu_model_t::set_command_id_at]
        fn set_command_id_at(&mut self, index: usize, command_id: i32) -> bool;

        /// See [cef_menu_model_t::get_label]
        fn get_label(&self, command_id: i32) -> CefString {
            get_label.and_then(|f| unsafe {
                CefString::from_userfree_cef(f(self.get_this(), command_id))
            })
        }

        /// See [cef_menu_model_t::get_label_at]
        fn get_label_at(&self, index: usize) -> CefString {
            get_label_at
                .and_then(|f| unsafe { CefString::from_userfree_cef(f(self.get_this(), index)) })
        }

        /// See [cef_menu_model_t::set_label]
        fn set_label(&mut self, command_id: i32, label: CefString) -> bool {
            set_label.map(|f| unsafe { f(self.get_this(), command_id, &label.as_raw()) == 1 })
        }

        /// See [cef_menu_model_t::set_label_at]
        fn set_label_at(&mut self, index: usize, label: CefString) -> bool {
            set_label_at.map(|f| unsafe { f(self.get_this(), index, &label.as_raw()) == 1 })
        }

        /// See [cef_menu_model_t::get_type]
        fn get_type(&self, command_id: i32) -> cef_sys::cef_menu_item_type_t;

        /// See [cef_menu_model_t::get_type_at]
        fn get_type_at(&self, index: usize) -> cef_sys::cef_menu_item_type_t;

        /// See [cef_menu_model_t::get_group_id]
        fn get_group_id(&self, command_id: i32) -> i32;

        /// See [cef_menu_model_t::get_group_id_at]
        fn get_group_id_at(&self, index: usize) -> i32;

        /// See [cef_menu_model_t::set_group_id]
        fn set_group_id(&mut self, command_id: i32, group_id: i32) -> bool;

        /// See [cef_menu_model_t::set_group_id_at]
        fn set_group_id_at(&mut self, index: usize, group_id: i32) -> bool;

        /// See [cef_menu_model_t::get_sub_menu]
        fn get_sub_menu(&self, command_id: i32) -> CefMenuModel {
            get_sub_menu.and_then(|f| unsafe {
                let m = f(self.get_this(), command_id);
                if m.is_null() {
                    None
                } else {
                    CefMenuModel::from(m).into()
                }
            })
        }

        /// See [cef_menu_model_t::get_sub_menu_at]
        fn get_sub_menu_at(&self, index: usize) -> CefMenuModel {
            get_sub_menu_at.and_then(|f| unsafe {
                let m = f(self.get_this(), index);
                if m.is_null() {
                    None
                } else {
                    CefMenuModel::from(m).into()
                }
            })
        }

        /// See [cef_menu_model_t::is_visible]
        fn is_visible(&self, command_id: i32) -> bool;

        /// See [cef_menu_model_t::is_visible_at]
        fn is_visible_at(&self, index: usize) -> bool;

        /// See [cef_menu_model_t::set_visible]
        fn set_visible(&mut self, command_id: i32, visible: bool) -> bool;

        /// See [cef_menu_model_t::set_visible_at]
        fn set_visible_at(&mut self, index: usize, visible: bool) -> bool;

        /// See [cef_menu_model_t::is_enabled]
        fn is_enabled(&self, command_id: i32) -> bool;

        /// See [cef_menu_model_t::is_enabled_at]
        fn is_enabled_at(&self, index: usize) -> bool;

        /// See [cef_menu_model_t::set_enabled]
        fn set_enabled(&mut self, command_id: i32, enabled: bool) -> bool;

        /// See [cef_menu_model_t::set_enabled_at]
        fn set_enabled_at(&mut self, index: usize, enabled: bool) -> bool;

        /// See [cef_menu_model_t::is_checked]
        fn is_checked(&self, command_id: i32) -> bool;

        /// See [cef_menu_model_t::is_checked_at]
        fn is_checked_at(&self, index: usize) -> bool;

        /// See [cef_menu_model_t::set_checked]
        fn set_checked(&mut self, command_id: i32, checked: bool) -> bool;

        /// See [cef_menu_model_t::set_checked_at]
        fn set_checked_at(&mut self, index: usize, checked: bool) -> bool;

        /// See [cef_menu_model_t::has_accelerator]
        fn has_accelerator(&self, command_id: i32) -> bool;

        /// See [cef_menu_model_t::has_accelerator_at]
        fn has_accelerator_at(&self, index: usize) -> bool;

        /// See [cef_menu_model_t::set_accelerator]
        fn set_accelerator(
            &mut self,
            command_id: i32,
            key_code: i32,
            shift_pressed: bool,
            ctrl_pressed: bool,
            alt_pressed: bool,
        ) -> bool;

        /// See [cef_menu_model_t::set_accelerator_at]
        fn set_accelerator_at(
            &mut self,
            index: usize,
            key_code: i32,
            shift_pressed: bool,
            ctrl_pressed: bool,
            alt_pressed: bool,
        ) -> bool;

        /// See [cef_menu_model_t::remove_accelerator]
        fn remove_accelerator(&mut self, command_id: i32) -> bool;

        /// See [cef_menu_model_t::remove_accelerator_at]
        fn remove_accelerator_at(&mut self, index: usize) -> bool;

        // See [cef_menu_model_t::get_accelerator]
        // fn get_accelerator(&self, command_id: i32) -> (i32, bool, bool, bool) ;

        // See [cef_menu_model_t::get_accelerator_at]
        //fn get_accelerator_at(&self, index: usize) -> (i32, bool, bool, bool);

        /// See [cef_menu_model_t::set_color]
        fn set_color(
            &mut self,
            command_id: i32,
            color_type: cef_sys::cef_menu_color_type_t,
            color: cef_sys::cef_color_t,
        ) -> bool;

        /// See [cef_menu_model_t::set_color_at]
        fn set_color_at(
            &mut self,
            index: i32,
            color_type: cef_sys::cef_menu_color_type_t,
            color: cef_sys::cef_color_t,
        ) -> bool;

        /// See [cef_menu_model_t::get_color]
        fn get_color(&self, command_id: i32, color_type: crate::CefMenuColorType) -> u32 {
            get_color.and_then(|f| unsafe {
                let color = std::ptr::null_mut();
                if f(self.get_this(), command_id, color_type, color) == 1 {
                    if color.is_null() {
                        None
                    } else {
                        Some(*color)
                    }
                } else {
                    None
                }
            })
        }

        /// See [cef_menu_model_t::get_color_at]
        fn get_color_at(&self, index: i32, color_type: cef_sys::cef_menu_color_type_t) -> u32 {
            get_color_at.and_then(|f| unsafe {
                let color = std::ptr::null_mut();
                if f(self.get_this(), index, color_type, color) == 1 {
                    if color.is_null() {
                        None
                    } else {
                        Some(*color)
                    }
                } else {
                    None
                }
            })
        }

        /// See [cef_menu_model_t::set_font_list]
        fn set_font_list(&mut self, command_id: i32, font_list: CefString) -> bool {
            set_font_list
                .map(|f| unsafe { f(self.get_this(), command_id, &font_list.as_raw()) == 1 })
        }

        /// See [cef_menu_model_t::set_font_list_at]
        fn set_font_list_at(&mut self, index: i32, font_list: CefString) -> bool {
            set_font_list_at.map(|f| unsafe { f(self.get_this(), index, &font_list.as_raw()) == 1 })
        }
    );
}

pub trait MenuModelDelegate: Sized {
    fn into_raw(self) -> *mut cef_menu_model_delegate_t {
        todo!()
    }
}
