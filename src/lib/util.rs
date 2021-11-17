use std::fs;
use std::path::{Path, PathBuf};

pub fn get_full_path(p: PathBuf) -> PathBuf {
  let result = fs::canonicalize(&p);

  match result {
    Ok(path) => path,
    Err(_) => p,
  }
}

pub fn read_file_to_string(p: &PathBuf) -> Result<String, (i32, String)> {
  let content_result = fs::read(&p);
  match content_result {
    Ok(content_bytes) => {
      let string_result = String::from_utf8(content_bytes);
      match string_result {
        Ok(string) => {
          return Ok(string);
        }
        Err(e) => {
          return Err((
            1,
            format!(
              "Failed to parse {} content from bytes to string: {}",
              p.display(),
              e
            ),
          ));
        }
      }
    }
    Err(e) => {
      return Err((
        1,
        format!("Failed to read the contents of {}: {}", p.display(), e),
      ));
    }
  }
}

pub fn read_dir_recursive(p: &Path) -> Result<Vec<PathBuf>, String> {
  let mut all_files = vec![];

  let read_result = fs::read_dir(p);
  match read_result {
    Ok(entries) => {
      for entry in entries {
        match entry {
          Ok(entry) => {
            let path = entry.path();
            let metadata = fs::metadata(&path);
            match metadata {
              Ok(metadata) => {
                if metadata.is_file() {
                  all_files.push(path);
                } else if metadata.is_dir() {
                  let sub_files = read_dir_recursive(&path).unwrap();
                  all_files.extend(sub_files);
                }
              }
              Err(e) => {
                return Err(format!(
                  "Failed to read metadata of {}: {}",
                  path.display(),
                  e
                ));
              }
            }
          }
          Err(e) => {
            return Err(format!("Failed to read directory entry: {}", e));
          }
        }
      }

      return Ok(all_files);
    }
    Err(e) => {
      return Err(format!("Failed to read directory: {}", e));
    }
  }
}
