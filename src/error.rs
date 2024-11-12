#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("init failure: {0}")]
    CannotInit(i32),
}

pub type Result<T> = std::result::Result<T, Error>;
