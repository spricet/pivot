use crate::config::Config;
use crate::switch::block::BlockHandler;
use crate::switch::command::SwitcherCommandFactory;
use crate::switch::error::{Error, Result};

pub struct Switcher {
    config: Config,
}

impl Switcher {
    pub fn new(config: Config) -> Switcher {
        Switcher { config }
    }

    pub fn switch(&self, target_name: &str) -> Result<()> {
        let target = self.config.get_target(target_name).map_err(Error::from)?;
        let mut cmd = SwitcherCommandFactory::new_bash();

        for block in target.blocks.iter() {
            let handler = BlockHandler::new(block);
            handler.handle(block, &mut cmd).map_err(Error::from)?;
        }

        if let Some(post_init) = target.post_init {
            cmd.set_ps1(target_name, &post_init.prompt);
            if let Some(start_dir) = post_init.start_dir {
                cmd.set_start_dir(&start_dir);
            }
        }
        cmd.run().map_err(Error::from)
    }
}
