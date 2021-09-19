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

pub trait BlockHandler {
    fn handle(&self, block: &Block, cmd: &mut Box<dyn SwitcherCommand>) -> Result<()>;
}

pub struct BlockHandlerFactory;

impl BlockHandlerFactory {
    pub fn new(block: &Block) -> Box<dyn BlockHandler> {
        match block {
            Block::AwsProfile(_) => Box::new(AwsProfileBlockHandler::new()),
            Block::Kubeconfig(_) => Box::new(KubeconfigBlockHandler::new()),
        }
    }
}