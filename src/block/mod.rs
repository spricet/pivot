use crate::block::aws_profile::{AwsProfileBlock, AwsProfileBlockHandler};
use crate::block::error::Result;
use crate::block::kubeconfig::{KubeconfigBlock, KubeconfigBlockHandler};
use crate::switch::command::SwitcherCommand;
use serde::{Deserialize, Serialize};

pub mod aws_profile;
pub mod error;
pub mod kubeconfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Block {
    AwsProfile(AwsProfileBlock),
    Kubeconfig(KubeconfigBlock),
}

pub enum BlockHandler {
    AwsProfile(AwsProfileBlockHandler),
    Kubeconfig(KubeconfigBlockHandler),
}

impl BlockHandler {
    pub fn new(block: &Block) -> BlockHandler {
        match block {
            Block::AwsProfile(_) => BlockHandler::AwsProfile(AwsProfileBlockHandler {}),
            Block::Kubeconfig(_) => BlockHandler::Kubeconfig(KubeconfigBlockHandler {}),
        }
    }

    pub fn handle(&self, block: &Block, cmd: &mut Box<dyn SwitcherCommand>) -> Result<()> {
        match self {
            BlockHandler::AwsProfile(aws) => {
                if let Block::AwsProfile(ablock) = block {
                    return aws.handle(ablock, cmd);
                }
            }
            BlockHandler::Kubeconfig(kubeconfig) => {
                if let Block::Kubeconfig(kblock) = block {
                    return kubeconfig.handle(kblock, cmd);
                }
            }
        }
        Ok(())
    }
}
