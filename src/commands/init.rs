use crate::lib::command_manager::Command;

pub fn get_command() -> Command {
  Command {
    name: "init".to_string(),
    description: "Initialize a new Chum project".to_string(),
    examples: vec![".".to_string(), "./my_project".to_string()],
    allowed_args: vec![("The directory to initialize the project in".to_string(), true)],
    allowed_flags: vec![],
    handler: Box::new(|ctx| {
      let default_dir = ".".to_string();
      let dir = ctx.cli.args.first().unwrap_or(&default_dir);
      
      // TODO: Check if the directory exists
      // TODO: Check if the directory is empty
      // TODO: Check if the directory is a directory
      // TODO: Create new directory
      // TODO: Create new .chum file
      // TODO: Create initial compressed files

      Ok(0)
    })
  }
}