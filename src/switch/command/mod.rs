use crate::switch::command::bash::BashSwitcherCommand;
use crate::switch::command::error::Result;
use crate::config::Prompt;

pub mod bash;
pub mod error;

pub trait SwitcherCommand {
    fn env(&mut self, key: &str, val: &str);
    fn set_ps1(&mut self, target_name: &str, prompt: &Prompt);
    fn run(&mut self) -> Result<()>;
}

pub struct SwitcherCommandFactory;

impl SwitcherCommandFactory {
    pub fn new_bash() -> Box<dyn SwitcherCommand> {
        Box::new(BashSwitcherCommand::new())
    }
}
