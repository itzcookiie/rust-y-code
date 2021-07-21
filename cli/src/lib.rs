use clap::Clap;


#[derive(Debug,Clap)]
pub struct PeopleCommand {
    #[clap(short, long)]
    pub first_name: String,
    #[clap(short, long)]
    pub last_name: String
}

