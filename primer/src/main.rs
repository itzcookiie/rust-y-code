use std::env;

fn main() {

    let names = vec!["fjrek","gjklkrje"];

    for (key, val) in env::vars() {
        println!("{}: {}", key, val);
    }

    println!("Hello, world!");
}
