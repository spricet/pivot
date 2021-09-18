pub mod aws;
pub mod kubernetes;
pub mod prompt;

use serde::{Serialize, Deserialize};
use crate::config::aws::AwsProfileBlock;
use crate::config::kubernetes::KubeconfigBlock;
use std::collections::HashMap;
use crate::config::prompt::{OverridePrompt, BuiltinPrompt};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    api_version: String,
    targets: Vec<Target>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    prompt: Prompt,
    start_dir: Option<String>,
    blocks: Vec<Block>,
    env: HashMap<String, String>,
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