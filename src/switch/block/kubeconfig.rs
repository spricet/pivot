use crate::config::kubernetes::KubeconfigBlock;
use crate::switch::block::error::Result;
use std::process::Command;

const KUBECONFIG_ENV: &str = "KUBECONFIG";

pub struct KubeconfigBlockHandler {}

impl KubeconfigBlockHandler {
    pub fn handle(&self, block: &KubeconfigBlock, cmd: &mut Command) -> Result<()> {
        cmd.env(KUBECONFIG_ENV, &block.kubeconfig);
        Ok(())
    }
}
