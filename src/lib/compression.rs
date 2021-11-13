use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn get_file_as_binary(file_path: &String) -> Vec<u8> {
  let path = Path::new(file_path);
  let mut file = File::open(path).unwrap();
  let mut contents = Vec::new();
  file.read_to_end(&mut contents).unwrap();
  contents
}

pub struct Compress {}

impl Compress {
  pub fn new(input: &Vec<u8>) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input).unwrap();
    encoder.finish().unwrap()
  }
}

pub struct Decompress {}

impl Decompress {
  pub fn new(input: Box<dyn Read>) -> String {
    let mut output = String::new();
    let mut decoder = ZlibDecoder::new(input);
    decoder.read_to_string(&mut output).unwrap();
    println!("{:?}", output);
    output
  }
}
