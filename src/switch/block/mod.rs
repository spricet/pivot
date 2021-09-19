use crate::config::Block;
use crate::switch::block::aws_profile::AwsProfileBlockHandler;
use crate::switch::block::error::Result;
use crate::switch::block::kubeconfig::KubeconfigBlockHandler;
use crate::switch::command::SwitcherCommand;

pub mod aws_profile;
pub mod error;
pub mod kubeconfig;

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
