use std::time::Instant;
use rayon::prelude::*;
use collections::*;

fn main() {


    let start = Instant::now();
    let colours  = vec![Color::Blue, Color::Green];
    let protocols  = vec![Protocol::Http, Protocol::Ws, Protocol::Udp];
    let countries  = vec![Country::US, Country::UK, Country::France, Country::Denmark];
    let numIDs  = ids();

    let par_iter:rayon::slice::Iter<i64> = numIDs.par_iter();

    let mut result = par_iter.flat_map(|numID|
        protocols.iter().flat_map(|protocol|
            countries.iter().flat_map(|country|
                colours.iter().map(|colour| Obj{
                    color: *colour,
                    protocol:*protocol,
                    country:*country,
                    id: *numID
                }).collect::<Vec<_>>()
            ).collect::<Vec<_>>()
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    println!("{}", result.len());
    println!("finished in {:?}",start.elapsed());

}