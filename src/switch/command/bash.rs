use crate::prompt::Prompt;
use crate::switch::command::error::{CommandError, Result};
use crate::switch::command::{SwitcherCommand, PIVOT_PS1_ENV, PIVOT_START_DIR_ENV};
use std::process::{Command, Stdio};
use crate::switch::command::expand;

const BASH_PATH: &str = "/bin/bash";
// const BASHRC_PATH: &str = ".bashrc";
// const HOME_ENV: &str = "HOME";
// const BASHRC_MARKER: &str = "### Pivot ###";
// const BASHRC_END_MARKER: &str = "### End Pivot ###";

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
                format!(
                    "\\e[01;31m[{prefix}]{target}:\\e[01;34m\\w\\e[0m\\$ ",
                    prefix = builtin.prefix,
                    target = target_name
                )
            }
            Prompt::Override(over) => over.prompt_override.clone(),
        };
        self.cmd.env(PIVOT_PS1_ENV, ps1);
    }

    fn set_start_dir(&mut self, start_dir: &str) {
        match expand::expand_path(start_dir) {
            Ok(path) => {
                self.cmd.env(PIVOT_START_DIR_ENV, path);
            },
            Err(e) => {
                println!("warning: could not expand start_dir: {} - {:?}", start_dir, e);
            }
        }
    }

    fn run(&mut self) -> Result<()> {
        let mut child = self
            .cmd
            .arg("-i")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(CommandError::from)?;
        let status = child.wait().map_err(CommandError::from)?;
        match status.success() {
            true => Ok(()),
            false => Err(CommandError::ExitStatusError(status.code())),
        }
    }
}


/*
pub fn configure_bash_rc() -> Result<()> {
    let bashrc = get_bashrc_path();
    if !bashrc.exists() {
        println!("warning '~/{}' doesn't exist, skipping bashrc configuration", BASHRC_PATH);
        return Ok(())
    }
    let bashrc = OpenOptions::new()
        .read(true)
        .write(true)
        .open(bashrc)
        .map_err(CommandError::from)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .map_err(CommandError::from)?;

    if contents.contains(BASHRC_MARKER) {
    } else {
    }
    Ok(())
}

fn get_bashrc_path() -> PathBuf {
    PathBuf::from(std::env::var(HOME_ENV).unwrap_or_default())
        .join(BASHRC_PATH)
}
*/
