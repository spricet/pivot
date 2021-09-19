use crate::config::error::{ConfigError, Result};
use crate::target::Target;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api_version: String,
    pub targets: Vec<Target>,
}

impl Config {
    pub fn get_target(&self, target_name: &str) -> Result<Target> {
        for target in self.targets.iter() {
            if target.name == target_name {
                return Ok(target.clone());
            }
        }
        Err(ConfigError::MissingTarget(target_name.to_string()))
    }
}
