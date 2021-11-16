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
      let target = Path::new(ctx.cli.args.first().unwrap_or(&default_dir));
      let chum_dir = Path::new(target).join(".chum");

      // Making sure the target is not a file
      if target.exists() {
        let dir_metadata = fs::metadata(&target);
        match dir_metadata {
          Ok(metadata) => {
            if metadata.is_file() {
              return Err((
                1,
                format!(
                  "{} is a file. Please specify a directory to initialize a chum project in",
                  target.display()
                ),
              ));
            }
          }
          Err(e) => {
            return Err((
              1,
              format!("Failed to read metadata of {}: {}", target.display(), e),
            ));
          }
        }
      }

      // Checking if a chum project already exists in the target directory
      if chum_dir.exists() {
        let chum_metadata = fs::metadata(&chum_dir).unwrap();
        if chum_metadata.is_dir() {
          if ctx.cli.flags.contains(&"force".to_string()) {
            let result = fs::remove_dir_all(&chum_dir);
            match result {
              Ok(_) => {
                display::info("Removed existing chum project due to 'force' flag".to_string());
              }
              Err(e) => {
                return Err((1, format!("Failed to remove existing chum project: {}", e)));
              }
            }
          } else {
            return Err((1, format!(
              "Chum project ({}) already exists. Use the 'force' flag to wipe and reinitialize the .chum directory",
              util::get_full_path(chum_dir.clone()).display()
            )));
          }
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

      // Creating the chum project directory
      let create_result = fs::create_dir_all(&chum_dir);
      match create_result {
        Ok(_) => {
          display::success(format!(
            "Created {}",
            util::get_full_path(chum_dir.clone()).display()
          ));
        }
        Err(e) => {
          return Err((1, format!("Failed to create {}: {}", chum_dir.display(), e)));
        }
      }

      // TODO: Create initial compressed files

      let files_result = util::read_dir_recursive(&target).unwrap();
      println!("{:?}", files_result);

      Ok(0)
    }),
  }
}
