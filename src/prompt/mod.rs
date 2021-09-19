pub mod builtin;
pub mod poverride;

use crate::prompt::builtin::BuiltinPrompt;
use crate::prompt::poverride::OverridePrompt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Prompt {
    Builtin(BuiltinPrompt),
    Override(OverridePrompt),
}
