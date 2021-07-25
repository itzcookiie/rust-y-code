use std::net::TcpStream;
use std::io::{Write, Read};

fn main() {

    let mut stream = match TcpStream::connect("localhost:8012") {
        Ok(s) => s,
        Err(e) => {
            println!("tcp error: {}", e);
            panic!();
        }
    };


    let mut request_data = String::from("hello");
    request_data.push_str("mate\r\n");
    stream.write_all(request_data.as_bytes()).unwrap();

}