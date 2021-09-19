use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("missing target {0}")]
    MissingTarget(String),
}
