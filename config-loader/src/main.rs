use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    age: u32,
    city: String,
}

fn main() {
    let data = fs::read_to_string("config.toml")
        .expect("Failed to read config file");

    let cfg: Config = toml::from_str(&data)
        .expect("Failed to parse TOML");

    println!("{:#?}", cfg);
}
