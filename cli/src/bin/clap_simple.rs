use clap;
use clap::{App, Arg};

fn main() {
    let cli_app = App::new("rustCLI").arg(
        Arg::new("Hello")
            .short('h')
            .long("hello")
            .takes_value(true)
    );

    let matches = cli_app.get_matches();

    if let Some(i) = matches.value_of("Hello") {
        println!("Value for input: {}", i);
    }
}