use sha1::{Digest, Sha1};
use std::fs;
use std::path::{Path, PathBuf};

pub fn sha1(s: &str) -> String {
  let mut hasher = Sha1::new();
  hasher.update(s.as_bytes());
  let res = hasher.finalize();
  format!("{:x}", res)
}

pub fn get_current_chum_dir() -> Result<PathBuf, (i32, String)> {
  let chum_path = Path::new("./.chum");
  if !chum_path.exists() {
    return Err((
          1,
          format!(
            "Your current working directory ({}) does not have a chum project initialized. Run 'chum init' to initialize a new project",
            get_full_path(std::env::current_dir().unwrap_or(PathBuf::new())).display())
        ));
  }

  let metadata_result = fs::metadata(chum_path);
  match metadata_result {
    Ok(metadata) => {
      if !metadata.is_dir() {
        return Err((
          1,
          format!("Your current chum project ({}) is not a directory. Remove the '.chum' file and run 'chum init .' to initialize a chum project", get_full_path(PathBuf::from(chum_path)).display())
        ));
      } else {
        return Ok(PathBuf::from(chum_path));
      }
    }
    Err(e) => {
      return Err((
        1,
        format!(
          "Failed to fetch metadata for your current chum project ({}): {}",
          get_full_path(PathBuf::from(chum_path)).display(),
          e
        ),
      ));
    }
  }
}

pub fn get_full_path(p: PathBuf) -> PathBuf {
  let result = fs::canonicalize(&p);

  match result {
    Ok(path) => path,
    Err(_) => p,
  }
}

pub fn read_file_to_string(p: &Path) -> Result<String, (i32, String)> {
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
