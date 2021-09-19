use thiserror::Error;

pub type Result<T, E = ConfigError> = std::result::Result<T, E>;

#[derive(Debug, Clone, Error)]
pub enum ConfigError {
    #[error("missing target {0}")]
    MissingTarget(String),
}
