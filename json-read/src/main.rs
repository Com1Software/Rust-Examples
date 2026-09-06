use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

fn main() {
    let data = fs::read_to_string("sample.json")
        .expect("Failed to read JSON file");

    let person: Person = serde_json::from_str(&data)
        .expect("Failed to parse JSON");

    println!("{:#?}", person);
}
