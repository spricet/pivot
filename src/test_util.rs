use crate::prompt::Prompt;
use crate::switch::command::SwitcherCommand;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn gen_str(length: usize, c: char) -> String {
    let mut s = String::from("");
    for _ in 0..length {
        s.push(c);
    }
    s
}

pub struct MockSwitcherCommand {
    pub env: Arc<Mutex<HashMap<String, String>>>,
}

impl MockSwitcherCommand {
    pub fn new(env: Arc<Mutex<HashMap<String, String>>>) -> Self {
        MockSwitcherCommand { env }
    }
}

impl SwitcherCommand for MockSwitcherCommand {
    fn env(&mut self, key: &str, val: &str) {
        let mut e = self.env.lock().unwrap();
        e.insert(key.to_string(), val.to_string());
    }

    fn set_ps1(&mut self, _target_name: &str, _prompt: &Prompt) {}

    fn set_start_dir(&mut self, _start_dir: &str) {}

    fn run(&mut self) -> crate::switch::command::error::Result<()> {
        Ok(())
    }
}
