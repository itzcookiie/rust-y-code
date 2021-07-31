use std::time::Instant;
use rayon;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let start = Instant::now();
    let client = reqwest::blocking::Client::new();

    let urls = (0..1000).into_iter().map(|_|"https://jsonplaceholder.typicode.com/todos/1".to_string()).collect::<Vec<_>>();
    let par_iter:rayon::slice::Iter<String> = urls.par_iter();

    par_iter.for_each(|x|{
        let response = client.get("https://jsonplaceholder.typicode.com/todos/1")
            .send()
            .unwrap()
            .text()
            .unwrap();
        println!("{:?}", response);
    });

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}


// Don't use for loops, use collections
// To speed up computation, use threads