use thiserror::Error;

pub type Result<T, E = CommandError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("exit status error: status code: {0:?}")]
    ExitStatusError(Option<i32>),
}
