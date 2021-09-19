use crate::block::error::Result;
use crate::block::{Block, BlockHandler};
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
    pub fn new() -> KubeconfigBlockHandler {
        KubeconfigBlockHandler {}
    }
}

impl Default for KubeconfigBlockHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockHandler for KubeconfigBlockHandler {
    fn handle(&self, block: &Block, cmd: &mut Box<dyn SwitcherCommand>) -> Result<()> {
        if let Block::Kubeconfig(kblock) = block {
            cmd.env(KUBECONFIG_ENV, &kblock.kubeconfig);
        }
        Ok(())
    }
}
