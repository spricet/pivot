use crate::config::aws::AwsProfileBlock;
use crate::switch::block::error::Result;
use crate::switch::command::SwitcherCommand;

const AWS_PROFILE_ENV: &str = "AWS_PROFILE";

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
