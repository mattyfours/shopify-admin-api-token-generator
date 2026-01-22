use serde_json;
use serde::de::DeserializeOwned;
use serde::Serialize;

use std::fs::File;

pub fn read_json_file<T: DeserializeOwned>(file_path: &str) -> T {
  let file = File::open(file_path).expect("Failed to open file");
  let error_message = format!("Failed to read file: {}", file_path);
  return serde_json::from_reader(file).expect(&error_message);
}


pub fn write_json_file<T: Serialize>(file_path: &str, data: &T) {
  let file = File::create(file_path).expect("Failed to create file");
  let error_message = format!("Failed to write file: {}", file_path);
  serde_json::to_writer_pretty(file, data).expect(&error_message)
}
