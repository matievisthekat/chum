use crate::lib::{command_manager::Command, display, util};
use std::fs;
use std::path::Path;

pub fn get_command() -> Command {
  Command {
    name: "init".to_string(),
    description: "Initialize a new Chum project".to_string(),
    examples: vec![".".to_string(), "./my_project".to_string()],
    allowed_args: vec![(
      "The directory to initialize the project in".to_string(),
      true,
    )],
    allowed_flags: vec!["force".to_string()],
    handler: Box::new(|ctx| {
      let default_dir = ".".to_string();
      let dir = ctx.cli.args.first().unwrap_or(&default_dir);
      let chum_dir = Path::new(dir).join(".chum");

      if chum_dir.exists() {
        let chum_metadata = fs::metadata(&chum_dir).unwrap();
        if chum_metadata.is_dir() && !ctx.cli.flags.contains(&"force".to_string()) {
          display::info(format!(
            "{} already exists. Use the 'force' flag to wipe and reinitialize the .chum directory",
            util::get_full_path(chum_dir.clone()).display()
          ));

          return Ok(0);
        } else if chum_metadata.is_file() {
          return Err((
            1,
            format!(
              "{} is a file! Please remove this file to initialize a chum project",
              util::get_full_path(chum_dir.clone()).display()
            ),
          ));
        }
      }

      fs::create_dir_all(&chum_dir).unwrap();
      display::success(format!(
        "Created {}",
        util::get_full_path(chum_dir.clone()).display()
      ));

      // TODO: Create new .chumrc file
      // TODO: Create initial compressed files

      Ok(0)
    }),
  }
}
