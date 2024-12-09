#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("init failure: {0}")]
    CannotInit(i32),
    #[error("process exit: {0}")]
    Exit(i32),
    #[error("cannot create browser view")]
    CannotCreateBrowserView,
    #[cfg(windows)]
    #[error("windows os error: {0}")]
    WinOs(windows::core::Error),
    #[error("cannot create browser")]
    CannotCreateBrowser,
    #[error("null ptr")]
    NullPtr,
    #[error("raw: {0:?}")]
    Raw(Option<crate::string::CefString>),
}

pub type Result<T> = std::result::Result<T, Error>;
