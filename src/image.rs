use crate::prelude::*;

/// See [cef_image_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefImage(cef_image_t);

impl CefImage {
    /// See [cef_sys::cef_image_create]
    pub fn create(_image: image::DynamicImage) -> Result<Self> {
        let image = unsafe { cef_sys::cef_image_create() };
        Ok(Self::from(image))
    }

    wrapper_methods!(
        /// See [cef_image_t::is_empty]
        fn is_empty(&self) -> bool;
        /// See [cef_image_t::get_width]
        fn get_width(&self) -> usize;
        /// See [cef_image_t::get_height]
        fn get_height(&self) -> usize;
    );
}
