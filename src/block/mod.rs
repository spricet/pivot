use crate::block::aws_profile::{AwsProfileBlock, AwsProfileBlockHandler};
use crate::block::error::Result;
use crate::block::kubeconfig::{KubeconfigBlock, KubeconfigBlockHandler};
use crate::switch::command::SwitcherCommand;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

pub mod aws_assume_role;
pub mod aws_profile;
pub mod error;
pub mod kubeconfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum Block {
    AwsProfile(AwsProfileBlock),
    // AwsAssumeRole(),
    Kubeconfig(KubeconfigBlock),
}

impl Validate for Block {
    fn validate(&self) -> std::result::Result<(), ValidationErrors> {
        match self {
            Block::AwsProfile(a) => a.validate(),
            Block::Kubeconfig(k) => k.validate(),
        }
    }
}

pub trait BlockHandler {
    fn handle(&self, block: &Block, cmd: &mut Box<dyn SwitcherCommand>) -> Result<()>;
}

pub struct BlockHandlerFactory;

impl BlockHandlerFactory {
    pub fn new_from_block(block: &Block) -> Box<dyn BlockHandler> {
        match block {
            Block::AwsProfile(_) => Box::new(AwsProfileBlockHandler::new()),
            Block::Kubeconfig(_) => Box::new(KubeconfigBlockHandler::new()),
        }
    }
}
