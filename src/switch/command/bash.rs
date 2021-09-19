use crate::switch::command::error::{Error, Result};
use crate::switch::command::SwitcherCommand;
use std::process::{Command, Stdio};
use crate::config::Prompt;

const BASH_PATH: &str = "/bin/bash";
const PS1_OVERRIDE_ENV: &str = "PS1_OVERRIDE";
const START_DIR_ENV: &str = "START_DIR";

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

    fn set_ps1(&mut self, target_name: &str, prompt: &Prompt) {
        let ps1 = match prompt {
            Prompt::Builtin(builtin) => {
                format!("\\e[01;31m[{prefix}]{target}:\\e[01;34m\\w\\e[0m\\$ ", prefix = builtin.prefix, target = target_name)
            }
            Prompt::Override(over) => {
                over.prompt_override.clone()
            }
        };
        self.cmd.env(
            PS1_OVERRIDE_ENV,
            ps1
        );
    }

    fn set_start_dir(&mut self, start_dir: &str) {
        self.cmd.env(START_DIR_ENV, start_dir);
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
