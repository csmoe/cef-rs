use crate::prelude::*;
use crate::CefAxisAlignment;

/// See [cef_layout_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefLayout(cef_layout_t);

impl CefLayout {
    wrapper_methods!(
        /// See [cef_layout_t::as_box_layout]
        fn as_box_layout(&self) -> CefBoxLayout {
            self.0.as_box_layout.and_then(|f| unsafe {
                let box_layout = f(self.0.get_this());
                if box_layout.is_null() {
                    None
                } else {
                    CefBoxLayout::from_raw(box_layout).into()
                }
            })
        }

        /// See [cef_layout_t::as_fill_layout]
        fn as_fill_layout(&self) -> CefFillLayout {
            self.0.as_fill_layout.and_then(|f| unsafe {
                let fill_layout = f(self.0.get_this());
                if fill_layout.is_null() {
                    None
                } else {
                    CefFillLayout::from_raw(fill_layout).into()
                }
            })
        }

        /// See [cef_layout_t::is_valid]
        fn is_valid(&self) -> bool;
    );
}

/// See [cef_fill_layout_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefFillLayout(cef_fill_layout_t);

/// See [cef_box_layout_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefBoxLayout(cef_box_layout_t);

impl CefBoxLayout {
    wrapper_methods!(
        /// See [cef_box_layout_t::set_flex_for_view]
        fn set_flex_for_view(&self, view: crate::CefView, flex: i32);

        /// See [cef_box_layout_t::clerar_flex_for_view]
        fn clear_flex_for_view(&self, view: crate::CefView);
    );
}

#[derive(Copy, Clone)]
/// See [cef_box_layout_settings_t] for more docs.
pub struct CefBoxLayoutSettings {
    /// See [cef_box_layout_settings_t::horizontal]
    pub horizontal: bool,
    /// See [cef_box_layout_settings_t::inside_border_horizontal_spacing]
    pub inside_border_horizontal_spacing: i32,
    /// See [cef_box_layout_settings_t::inside_border_vertical_spacing]
    pub inside_border_vertical_spacing: i32,
    /// See [cef_box_layout_settings_t::inside_border_insets]
    pub inside_border_insets: crate::CefInsets,
    /// See [cef_box_layout_settings_t::between_child_spacing]
    pub between_child_spacing: i32,
    /// See [cef_box_layout_settings_t::main_axis_alignment]
    pub main_axis_alignment: CefAxisAlignment,
    /// See [cef_box_layout_settings_t::cross_axis_alignment]
    pub cross_axis_alignment: CefAxisAlignment,
    /// See [cef_box_layout_settings_t::minimum_cross_axis_size]
    pub minimum_cross_axis_size: i32,
    /// See [cef_box_layout_settings_t::default_flex]
    pub default_flex: u32,
}

impl CefBoxLayoutSettings {
    pub fn into_raw(self) -> cef_box_layout_settings_t {
        cef_box_layout_settings_t {
            horizontal: self.horizontal.into(),
            inside_border_horizontal_spacing: self.inside_border_horizontal_spacing,
            inside_border_vertical_spacing: self.inside_border_vertical_spacing,
            inside_border_insets: self.inside_border_insets,
            between_child_spacing: self.between_child_spacing,
            main_axis_alignment: self.main_axis_alignment,
            cross_axis_alignment: self.cross_axis_alignment,
            minimum_cross_axis_size: self.minimum_cross_axis_size,
            default_flex: self.default_flex as _,
        }
    }
}
