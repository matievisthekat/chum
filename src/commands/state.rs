use crate::lib::{
  command_manager::Command,
  display,
  state::{get_state_of_file, State},
  util,
};
use std::collections::HashMap;
use std::path::Path;
use termcolor::Color::Yellow;

pub fn get_command() -> Command {
  return Command {
    name: "state".to_string(),
    description: "Check the state of your current project".to_string(),
    examples: vec![],
    allowed_args: vec![],
    allowed_flags: vec![],
    handler: Box::new(|_ctx| {
      let chum_dir_result = util::get_current_chum_dir();
      let chum_dir = match chum_dir_result {
        Ok(dir) => dir,
        Err(e) => {
          return Err(e);
        }
      };

      let files_result = util::read_dir_recursive(Path::new("."));
      let files = match files_result {
        Ok(files) => match util::filter_ignored(files.clone()) {
          Ok(filtered) => filtered,
          Err(e) => {
            return Err(e);
          }
        },
        Err(e) => {
          return Err((1, format!("Failed to read current directory: {}", e)));
        }
      };

      let mut files_by_state: HashMap<State, Vec<String>> = HashMap::new();

      files_by_state.insert(State::Created, Vec::new());
      files_by_state.insert(State::Modified, Vec::new());
      files_by_state.insert(State::NoChange, Vec::new());
      files_by_state.insert(State::Deleted, Vec::new());
      files_by_state.insert(State::Irrelevant, Vec::new());

      for file in files {
        let state = match get_state_of_file(Path::new(&file), &chum_dir) {
          Ok(state) => state,
          Err(e) => {
            return Err(e);
          }
        };

        files_by_state
          .get_mut(&state)
          .unwrap()
          .push(file.display().to_string());
      }

      files_by_state.keys().for_each(|state| {
        let files = files_by_state.get(state).unwrap();

        if files.len() > 0 {
          display::info(format!("{}", state.to_string()));
          display::writeln(Yellow, format!("[\n\t{}\n]", files = files.join("\n\t")))
        }
      });

      return Ok(0);
    }),
  };
}
