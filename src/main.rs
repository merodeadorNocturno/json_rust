extern crate serde_json;
extern crate regex_syntax;

use serde_json::Value as JsonValue;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::time::Instant;
// use regex_syntax::
mod mods;

use mods::my_path::get_dir_path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Currency {
  name: String,
  symbol: String,
  position: u32,
  slug: String,
  tokens: Vec<String>,
  id: u32,
}

pub fn set_value(filepath: &PathBuf) -> Vec<JsonValue> {
  let data = read_file(filepath);
  let v: Vec<JsonValue> = serde_json::from_str(&data).unwrap();
  v
}

pub fn set_tokens(tokens: String) -> Vec<String> {
  let my_tokens: Vec<String> = serde_json::from_str(&tokens).unwrap();
  my_tokens
}

pub fn convert_to_string(my_value: &JsonValue, my_reference: String) -> String {
  my_value[my_reference].as_str().unwrap().to_string()
}

pub fn get_untyped_json(filepath: &PathBuf) -> Vec<Currency> {
  let _v: Vec<JsonValue> = set_value(filepath);
  
  let my_vec: Vec<Currency> = _v
    .into_iter()
    .map(|currency: JsonValue| Currency {
      name: convert_to_string(&currency, "name".to_string()),
      symbol: convert_to_string(&currency, "symbol".to_string()),
      position: currency["rank"].to_string().parse().unwrap(),
      slug: convert_to_string(&currency, "slug".to_string()),
      tokens: set_tokens(currency["tokens"].to_string()),
      id: currency["id"].to_string().parse().unwrap(),
    })
    .collect();

  my_vec
}

pub fn read_file(filepath: &PathBuf) -> String {
  let file = File::open(filepath).expect("could not open file::");
  let mut buffered_reader = BufReader::new(file);
  let mut contents = String::new();
  let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
    Ok(_number_of_bytes) => _number_of_bytes,
    Err(_err) => 0,
  };

  contents
}

fn main() {
  let init = Instant::now();
  let name: String = "currencies".to_string();
  let extension: String = "json".to_string();
  let file_path = get_dir_path(name, extension);
  println!("PATH {:?}", &file_path);
  let v: Vec<Currency> =
    get_untyped_json(&file_path);
  let v_120: Vec<Currency> = v
    .into_iter()
    .filter(|coin| coin.name.to_string() == "XRP")
    .collect();

  let ended = Instant::now();
  println!("Rust: {:?}", ended.duration_since(init));
  // println!("v_120 len: {}", &v_120.len());
  // println!("coins {:?}", &v_120[0].name);
}
