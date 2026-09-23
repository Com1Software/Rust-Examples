use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

fn main() {
    // Write a simple text file
    let text = "Hello from Rust!\nThis file was written using std::fs::write.";
    fs::write("output.txt", text).expect("Failed to write text file");

    println!("Wrote output.txt");

    // Write a JSON file
    let person = Person {
        name: "Dave".to_string(),
        age: 55,
        city: "Hudson".to_string(),
    };

    let json = serde_json::to_string_pretty(&person)
        .expect("Failed to serialize JSON");

    fs::write("person.json", json).expect("Failed to write JSON file");

    println!("Wrote person.json");
}
