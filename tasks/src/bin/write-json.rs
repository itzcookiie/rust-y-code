use std::io;
use std::fs::File;
use std::io::{Write, Bytes};

fn main() {
    let response = reqwest::blocking::get("https://jsonplaceholder.typicode.com/todos/1")
        .unwrap()
        .text()
        .unwrap();

    let mut file = File::create("./tasks/test.json").unwrap();
    file.write_all(response.as_bytes());
}
