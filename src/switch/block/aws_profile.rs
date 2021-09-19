use crate::config::aws::AwsProfileBlock;
use crate::switch::block::error::Result;
use std::process::Command;

const AWS_PROFILE_ENV: &str = "AWS_PROFILE";

pub struct AwsProfileBlockHandler {}

impl AwsProfileBlockHandler {
    pub fn handle(&self, block: &AwsProfileBlock, cmd: &mut Command) -> Result<()> {
        cmd.env(AWS_PROFILE_ENV, &block.profile);
        Ok(())
    }
}