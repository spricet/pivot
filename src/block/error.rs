use thiserror::Error;
use crate::switch::command::error::CommandError;

pub type Result<T, E = BlockError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum BlockError {
    #[error("command error: {0}")]
    CommandError(#[from] CommandError)
}
