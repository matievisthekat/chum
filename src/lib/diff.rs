use crate::lib::compression;
use std::fs::read;
use std::path::Path;

pub fn is_modified(src: &Path, origin: &Path) -> Result<bool, (i32, String)> {
  let src_content = match read(src) {
    Ok(content) => content,
    Err(e) => {
      return Err((
        1,
        format!("Could not read source file ({}): {}", src.display(), e),
      ))
    }
  };

  let origin_content = match read(origin) {
    Ok(content) => content,
    Err(e) => {
      return Err((
        1,
        format!("Could not read origin file ({}): {}", origin.display(), e),
      ))
    }
  };

  let origin_decompressed = compression::decompress(&origin_content);

  Ok(src_content != origin_decompressed)
}
