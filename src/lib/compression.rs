use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

#[allow(dead_code)]
pub fn get_file_as_binary(file_path: &String) -> Vec<u8> {
  let path = Path::new(file_path);
  let mut file = File::open(path).unwrap();
  let mut contents = Vec::new();
  file.read_to_end(&mut contents).unwrap();
  contents
}

#[allow(dead_code)]
pub fn compress(input: &Vec<u8>) -> Vec<u8> {
  let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
  encoder.write_all(input).unwrap();
  encoder.finish().unwrap()
}

#[allow(dead_code)]
pub fn decompress(input: &[u8]) -> Vec<u8> {
  let mut output = vec![];
  let mut decoder = ZlibDecoder::new(input);
  decoder.read(&mut output).unwrap();
  output
}
