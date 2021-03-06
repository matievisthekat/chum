use crate::lib::compression;
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

      let origin = chum_dir.join("0");
      let create_origin_result = fs::create_dir_all(&origin);
      match create_origin_result {
        Ok(_) => {
          let files_result = util::read_dir_recursive(&target);
          let mut files_filtered = vec![];
          match files_result {
            Ok(files) => {
              let result = util::filter_ignored(files);
              match result {
                Ok(filtered) => {
                  filtered.into_iter().for_each(|f| files_filtered.push(f));
                }
                Err(e) => return Err(e),
              }
            }
            Err(e) => {
              return Err((
                1,
                format!("Failed to read target directory contents: {}", e),
              ));
            }
          }

          for file_path in files_filtered {
            let hashed_filename = util::sha1(file_path.file_name().unwrap().to_str().unwrap());
            let new_file_path = origin.join(hashed_filename);
            let read_result = fs::read(&file_path);

            match read_result {
              Ok(bytes) => {
                let compressed_bytes = compression::compress(&bytes);
                let write_result = fs::write(&new_file_path, compressed_bytes);
                match write_result {
                  Ok(_) => {}
                  Err(e) => {
                    return Err((
                      1,
                      format!(
                        "Failed to write compressed contents of {} to {}: {}",
                        file_path.display(),
                        new_file_path.display(),
                        e
                      ),
                    ))
                  }
                }
              }
              Err(e) => {
                return Err((
                  1,
                  format!("Failed to read file {}: {}", file_path.display(), e),
                ))
              }
            }
          }
        }
        Err(e) => return Err((1, format!("Failed to create origin directory: {}", e))),
      }

      Ok(0)
    }),
  }
}
