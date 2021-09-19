use crate::block::error::Result;
use crate::switch::command::SwitcherCommand;
use serde::{Deserialize, Serialize};
use crate::block::{BlockHandler, Block};

const AWS_PROFILE_ENV: &str = "AWS_PROFILE";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwsProfileBlock {
    pub profile: String,
}

pub struct AwsProfileBlockHandler {}

impl AwsProfileBlockHandler {
    pub fn new() -> AwsProfileBlockHandler {
        AwsProfileBlockHandler{}
    }
}

impl Default for AwsProfileBlockHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockHandler for AwsProfileBlockHandler {
    fn handle(&self, block: &Block, cmd: &mut Box<dyn SwitcherCommand>) -> Result<()> {
        if let Block::AwsProfile(ablock) = block {
            cmd.env(AWS_PROFILE_ENV, &ablock.profile);
        }
        Ok(())
    }
}