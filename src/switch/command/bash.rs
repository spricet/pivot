use crate::switch::command::error::{Error, Result};
use crate::switch::command::SwitcherCommand;
use std::process::{Command, Stdio};

const BASH_PATH: &str = "/bin/bash";

pub struct BashSwitcherCommand {
    cmd: Command,
}

impl BashSwitcherCommand {
    pub fn new() -> BashSwitcherCommand {
        BashSwitcherCommand {
            cmd: Command::new(BASH_PATH),
        }
    }
}

impl Default for BashSwitcherCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl SwitcherCommand for BashSwitcherCommand {
    fn env(&mut self, key: &str, val: &str) {
        self.cmd.env(key, val);
    }

    fn set_ps1(&mut self) {
        self.cmd.env(
            "PS1_OVERRIDE",
            "\\e[01;31m[test]my-target:\\e[01;34m\\w\\e[0m\\$ ",
        );
    }

    fn run(&mut self) -> Result<()> {
        let mut child = self
            .cmd
            .arg("-i")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(Error::from)?;
        let status = child.wait().map_err(Error::from)?;
        match status.success() {
            true => Ok(()),
            false => Err(Error::ExitStatusError(status.code())),
        }
    }
}
