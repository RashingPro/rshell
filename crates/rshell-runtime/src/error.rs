use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Invalid component: {message}")]
    InvalidComponent { message: String },
    #[error("Config execution error:\n{inner}")]
    ConfigExecutionError { inner: mlua::Error },
    #[error("{message}")]
    Other { message: String }
}

impl From<mlua::Error> for Error {
    fn from(value: mlua::Error) -> Self {
        Self::ConfigExecutionError { inner: value }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
