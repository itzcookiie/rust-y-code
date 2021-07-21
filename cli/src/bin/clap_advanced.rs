use cli::PeopleCommand;
use clap::{Clap};
fn main() {


    let pc: PeopleCommand = PeopleCommand::parse();
    println!("{:?}",pc);
    PeopleCommand::hello();

}