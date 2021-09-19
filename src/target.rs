use crate::block::Block;
use crate::pinit::PostInit;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub name: String,
    pub blocks: Vec<Block>,
    pub env: HashMap<String, String>,
    pub post_init: Option<PostInit>,
}
