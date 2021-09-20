use std::process::Command;
use crate::switch::command::error::{Result, CommandError};

pub fn expand_path(path: &str) -> Result<String> {
    let out = Command::new("ls")
        .arg("-d")
        .arg(path)
        .output()
        .map_err(CommandError::from)?;
    if !out.status.success() {
        return Err(CommandError::ExitStatusError(out.status.code()))
    }
    let expanded = String::from_utf8(out.stdout)
        .map_err(CommandError::from)?
        .trim()
        .to_string();
    Ok(expanded)
}