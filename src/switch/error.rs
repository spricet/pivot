use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("config error: {0}")]
    ConfigError(#[from] crate::config::error::Error),

    #[error("block error: {0}")]
    BlockError(#[from] crate::switch::block::error::Error),

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("exit status error: status code: {0:?}")]
    ExitStatusError(Option<i32>),
}
