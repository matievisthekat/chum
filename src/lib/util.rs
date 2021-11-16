use std::fs;
use std::path::PathBuf;

pub fn get_full_path(p: PathBuf) -> PathBuf {
  let result = fs::canonicalize(&p);

  match result {
    Ok(path) => path,
    Err(_) => p,
  }
}
