pub mod aws;
pub mod error;
pub mod kubernetes;
pub mod prompt;

use crate::config::aws::AwsProfileBlock;
use crate::config::error::{Error, Result};
use crate::config::kubernetes::KubeconfigBlock;
use crate::config::prompt::{BuiltinPrompt, OverridePrompt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api_version: String,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub name: String,
    pub prompt: Prompt,
    pub start_dir: Option<String>,
    pub blocks: Vec<Block>,
    pub env: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Prompt {
    Builtin(BuiltinPrompt),
    Override(OverridePrompt),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Block {
    AwsProfile(AwsProfileBlock),
    Kubeconfig(KubeconfigBlock),
}

impl Config {
    pub fn get_target(&self, target_name: &str) -> Result<Target> {
        for target in self.targets.iter() {
            if target.name == target_name {
                return Ok(target.clone());
            }
        }
        Err(Error::MissingTarget(target_name.to_string()))
    }
}
