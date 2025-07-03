use std::env;

use std::fs;
use std::path::Path;

fn load_tasks() -> Vec<String> {
    let path = "todo.json";
    if !Path::new(path).exists() {
        return Vec::new();
    }

    let data = fs::read_to_string(path).unwrap_or_else(|_| String::from("[]"));
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
