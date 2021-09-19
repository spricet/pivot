use thiserror::Error;

pub type Result<T, E = BlockError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum BlockError {}
