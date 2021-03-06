use std::time::Instant;

fn main() {
    let start = Instant::now();
    let client = reqwest::blocking::Client::new();
    for n in 0..1000 {
        let response = client.get("https://jsonplaceholder.typicode.com/todos/1")
            .send()
            .unwrap()
            .text()
            .unwrap();
        println!("{:?}", response);
    }
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
