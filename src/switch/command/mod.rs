use crate::prompt::Prompt;
use crate::switch::command::bash::BashSwitcherCommand;
use crate::switch::command::error::Result;

pub mod bash;
pub mod error;
pub mod expand;

const PIVOT_PS1_ENV: &str = "PIVOT_PS1";
const PIVOT_START_DIR_ENV: &str = "PIVOT_START_DIR";

pub trait SwitcherCommand {
    fn env(&mut self, key: &str, val: &str);
    fn set_ps1(&mut self, target_name: &str, prompt: &Prompt);
    fn set_start_dir(&mut self, start_dir: &str);
    fn run(&mut self) -> Result<()>;
}

pub struct SwitcherCommandFactory;

impl SwitcherCommandFactory {
    pub fn new_bash() -> Box<dyn SwitcherCommand> {
        Box::new(BashSwitcherCommand::new())
    }
}
