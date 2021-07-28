fn main() {
    let client = reqwest::blocking::Client::new();
    for n in 0..1000 {
        let response = client.get("https://jsonplaceholder.typicode.com/todos/1")
            .send()
            .unwrap()
            .text();
        println!("{:?}", response);
    }
}
