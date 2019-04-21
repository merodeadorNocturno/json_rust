extern crate serde_json;

use serde_json::Value as JsonValue;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::time::Instant;

pub struct Currency {
    name: String,
    symbol: String,
    position: u32,
    slug: String,
    tokens: Vec<String>,
    id: u32,
}

pub fn set_value(filepath: &str) -> Vec<JsonValue> {
    let data = read_file(filepath);
    let v: Vec<JsonValue> = serde_json::from_str(&data).unwrap();
    v
}

pub fn set_tokens(tokens: String) -> Vec<String> {
    let my_tokens: Vec<String> = serde_json::from_str(&tokens).unwrap();
    my_tokens
}

pub fn untyped_json(filepath: &str) -> Vec<Currency> {
    let mut my_vec: Vec<Currency> = Vec::new();    
    let v: Vec<JsonValue> = set_value(filepath);

    for currency in &v {
        let c = Currency {
            name: currency["name"].to_string(),
            symbol: currency["symbol"].to_string(),
            position: currency["rank"].to_string().parse().unwrap(),
            slug: currency["slug"].to_string(),
            tokens: set_tokens(currency["tokens"].to_string()),
            id: currency["id"].to_string().parse().unwrap(),
        };
        my_vec.push(c);
    }

    my_vec
}

pub fn read_file(filepath: &str) -> String {
    println!("Reading from {}", filepath);
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
    let v: Vec<Currency> = untyped_json("/Users/pills/Development/rust/read_json_from_file/json_files/currencies.json");
    let v_120: Vec<Currency> = v
        .into_iter()
        .filter(|c| c.id > 768 && c.id < 1899)
        .collect();

    for c in v_120 {
        let my_c: Currency = c;
        println!("{}", my_c.name);
    }

    let ended = Instant::now();
    println!("{:?}", ended.duration_since(init));
}
