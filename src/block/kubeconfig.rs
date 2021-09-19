use crate::block::error::Result;
use crate::switch::command::SwitcherCommand;
use serde::{Deserialize, Serialize};

const KUBECONFIG_ENV: &str = "KUBECONFIG";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KubeconfigBlock {
    pub kubeconfig: String,
}

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
