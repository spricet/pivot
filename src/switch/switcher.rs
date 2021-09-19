use crate::config::Config;
use crate::switch::block::BlockHandler;
use crate::switch::error::{Error, Result};
use std::process::{Command, Stdio};
use std::io::Write;

pub struct Switcher {
    config: Config,
}

impl Switcher {
    pub fn new(config: Config) -> Switcher {
        Switcher { config }
    }

    pub fn switch(&self, target_name: &str) -> Result<()> {
        let target = self.config.get_target(target_name).map_err(Error::from)?;
        let mut cmd = Command::new("/bin/bash");

        for block in target.blocks.iter() {
            let handler = BlockHandler::new(block);
            handler.handle(block, &mut cmd).map_err(Error::from)?;
        }

        cmd.env("PS1_OVERRIDE", "\\e[01;31m[test]my-target:\\e[01;34m\\w\\e[0m\\$ ");

        // let status = cmd
        //     .arg("-i")
        //     .stdin(Stdio::inherit())
        //     .stdout(Stdio::inherit())
        //     .stderr(Stdio::inherit())
        //     .status()
        //     .map_err(Error::from)?;
        let mut child = cmd
            .arg("-i")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(Error::from)?;
        let status = child.wait()
            .map_err(Error::from)?;
        match status.success() {
            true => Ok(()),
            false => Err(Error::ExitStatusError(status.code()))
        }
    }
}
