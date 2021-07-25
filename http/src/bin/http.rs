use std::net::TcpStream;
use std::io::{Write, Read};
use http::ExampleRequest;

fn main() {

    let e = ExampleRequest{ name: "yo".to_string() };
    let client = reqwest::blocking::Client::new();
    let r = client.post("http://localhost:8082")
        .json::<ExampleRequest>(&e)
        .send()
        .unwrap();

    println!("{:?}",r);
}