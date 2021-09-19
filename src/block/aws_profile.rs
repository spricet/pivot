use crate::block::error::Result;
use crate::switch::command::SwitcherCommand;
use serde::{Deserialize, Serialize};

const AWS_PROFILE_ENV: &str = "AWS_PROFILE";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwsProfileBlock {
    pub profile: String,
}

pub struct AwsProfileBlockHandler {}

impl AwsProfileBlockHandler {
    pub fn handle(
        &self,
        block: &AwsProfileBlock,
        cmd: &mut Box<dyn SwitcherCommand>,
    ) -> Result<()> {
        cmd.env(AWS_PROFILE_ENV, &block.profile);
        Ok(())
    }
}
