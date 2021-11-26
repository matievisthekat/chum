use crate::lib::{diff, util};
use std::path::{Path, PathBuf};

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum State {
  Irrelevant,
  NoChange,
  Modified,
  Created,
  Deleted,
}

impl std::fmt::Display for State {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self)
    // or, alternatively:
    // fmt::Debug::fmt(self, f)
  }
}

pub fn get_state_of_file(path: &Path, chum_dir: &PathBuf) -> Result<State, (i32, String)> {
  if path.is_file() {
    let hashed_name = util::sha1(path.file_name().unwrap().to_str().unwrap());
    let origin_path = chum_dir.join(format!("0/{}", hashed_name));

    if !path.exists() && origin_path.exists() {
      return Ok(State::Deleted);
    }

    if !origin_path.exists() {
      return Ok(State::Created);
    }

    let modified = diff::is_modified(path, &origin_path);

    match modified {
      Ok(true) => return Ok(State::Modified),
      Ok(false) => return Ok(State::NoChange),
      Err(e) => return Err(e),
    }
  } else {
    return Ok(State::Irrelevant);
  }
}
