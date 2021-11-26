use crate::lib::{diff, util};
use std::path::{Path, PathBuf};

#[derive(Hash, Eq, PartialEq)]
pub enum State {
  Irrelevant,
  NoChange,
  Modified,
  Created,
  Deleted,
}

pub fn get_state_of_file(path: &Path, chum_dir: &PathBuf) -> Result<State, (i32, String)> {
  let mut state = State::Irrelevant;
  if path.is_file() {
    let hashed_name = util::sha1(path.file_name().unwrap().to_str().unwrap());
    let origin_path = chum_dir.join(format!("0/{}", hashed_name));

    if !path.exists() && origin_path.exists() {
      state = State::Deleted;
    }

    if path.exists() && !origin_path.exists() {
      state = State::Created;
    }

    let modified = diff::is_modified(path, &origin_path);

    match modified {
      Ok(true) => state = State::Modified,
      Ok(false) => state = State::NoChange,
      Err(e) => return Err(e),
    }
  }

  Ok(state)
}
