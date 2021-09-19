use crate::prompt::Prompt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct PostInit {
    pub prompt: Prompt,
    pub start_dir: Option<String>,
}
