use crate::config::kubernetes::KubeconfigBlock;
use crate::switch::block::error::Result;
use crate::switch::command::SwitcherCommand;

const KUBECONFIG_ENV: &str = "KUBECONFIG";

pub struct KubeconfigBlockHandler {}

impl KubeconfigBlockHandler {
    pub fn handle(
        &self,
        block: &KubeconfigBlock,
        cmd: &mut Box<dyn SwitcherCommand>,
    ) -> Result<()> {
        cmd.env(KUBECONFIG_ENV, &block.kubeconfig);
        Ok(())
    }
}
