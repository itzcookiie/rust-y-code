use clap::Clap;
use cli::PostCommand;
use serde;
use serde_json::{json, Value};

fn main() {
    let c : PostCommand = PostCommand::parse();
    if c.flag == "GET" {
        let response = reqwest::blocking::get(c.url).unwrap().text();

        println!("Response = {:?}", response);
    } else if c.flag == "POST" {
        let json : Value  = serde_json::from_str(&c.data).unwrap();

        let client = reqwest::blocking::Client::new();
        let response = client.post(c.url)
            .json::<Value>(&json)
            .send()
            .unwrap()
            .text();

        println!("Response = {:?}", response);
    }
}