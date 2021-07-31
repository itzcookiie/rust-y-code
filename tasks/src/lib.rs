use clap::Clap;

#[macro_use]
extern crate task_macro;

#[derive(PrintStruct)]
pub struct TestStruct {
    pub data: String
}

#[derive(Debug,Clap)]
pub struct Car {
    #[clap(short, long)]
    pub id : u64,
    #[clap(short, long)]
    pub make : String
}