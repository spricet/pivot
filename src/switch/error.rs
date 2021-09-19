use thiserror::Error;

pub type Result<T, E = SwitchError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum SwitchError {
    #[error("config error: {0}")]
    ConfigError(#[from] crate::config::error::ConfigError),

    #[error("block error: {0}")]
    BlockError(#[from] crate::block::error::BlockError),

    #[error("command error: {0}")]
    CommandError(#[from] crate::switch::command::error::CommandError),
}
