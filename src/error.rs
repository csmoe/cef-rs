#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("init failure: {0}")]
    CannotInit(i32),
    #[error("process exit: {0}")]
    Exit(i32),
    #[error("cannot create browser view")]
    CannotCreateBrowserView,
}

pub type Result<T> = std::result::Result<T, Error>;
