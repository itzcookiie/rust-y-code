use clap::Clap;
use serde::{Serialize, Deserialize};

#[derive(Debug,Clap)]
pub struct PeopleCommand {
    #[clap(short, long)]
    pub first_name: String,
    #[clap(short, long)]
    pub last_name: String
}

#[derive(Debug,Clap)]
pub struct AddCommand {
    #[clap(short, long)]
    pub first_number: u8,
    #[clap(short, long)]
    pub second_number: u8
}

impl AddCommand {
    pub fn addition(&self)->u8{
        self.second_number+self.first_number
    }
}

#[derive(Debug,Clap)]
pub struct GetCommand {
    #[clap(short, long)]
    pub url : String
}

#[derive(Debug,Clap, Serialize, Deserialize, Default)]
pub struct PostCommand {
    #[clap(short, long)]
    pub flag : String,
    #[clap(short, long)]
    pub url : String,
    #[clap(short, long, default_value = "{}")]
    pub data : String
}

#[derive(Debug)]
pub struct Body {
    pub first_name : String,
    pub last_name : String
}