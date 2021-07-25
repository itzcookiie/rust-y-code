use clap::Clap;
use cli::AddCommand;

fn main() {
    let ac : AddCommand = AddCommand::parse();
    println!("{:?}", ac.addition());
}