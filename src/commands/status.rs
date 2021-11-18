use crate::lib::command_manager::Command;
use crate::lib::util;
use std::path::{Path, PathBuf};

pub fn get_command() -> Command {
  return Command {
    name: "status".to_string(),
    description: "Check the status of your current project".to_string(),
    examples: vec![],
    allowed_args: vec![],
    allowed_flags: vec![],
    handler: Box::new(|ctx| {
      let chum_dir_result = util::get_current_chum_dir();
      match chum_dir_result {
        Ok(chum_dir) => {
          return Ok(0);
        }
        Err(e) => {
          return Err(e);
        }
      }
    }),
  };
}
