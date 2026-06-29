use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Config execution error:\n{inner}")]
    ConfigExecutionError { inner: mlua::Error },
    #[error("{message}")]
    Other { message: String }
}

pub type Result<T> = std::result::Result<T, Error>;
