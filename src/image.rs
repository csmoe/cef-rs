use crate::{rc::RefGuard, Result};
use cef_sys::cef_image_t;
use cef_wrapper_macro::wrapper_methods;

crate::wrapper! {
    /// See [`cef_image_t`] for more docs.
    #[derive(Debug, Clone)]
    pub struct Image(cef_image_t);
}

impl Image {
    pub fn create(image: image::DynamicImage) -> Result<Self> {
        let image = unsafe { cef_sys::cef_image_create() };
        Ok(Self(unsafe { RefGuard::from_raw(image) }))
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
