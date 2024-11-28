use crate::prelude::*;
use crate::wrapper;

wrapper! {
    /// See [cef_post_data_t] for more docs.
    #[derive(Debug, Clone)]
    pub struct PostData(cef_post_data_t);
}

impl PostData {
    pub fn create() -> Result<PostData> {
        let ptr = unsafe { cef_post_data_create() };
        if ptr.is_null() {
            Err(Error::NullPtr)
        } else {
            Ok(unsafe { PostData::from_raw(ptr) })
        }
    }
}
