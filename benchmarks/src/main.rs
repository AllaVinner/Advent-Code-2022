use clap::{Parser};
use clap;
use std::string::String;

#[derive(Parser, Debug)]
#[command(author, version, 
    about="THis is a great command", 
    long_about = "Actually This is the greatest that ever existed")]
struct Args {
    #[arg(short, long)]
    day: u32,

    #[arg(short, long, default_value_t=1)]
    task: u32,

    #[arg(short, long, default_value_t=1)]
    benchmark_id: u32,
    
    #[arg(short, long)]
    input_file: String,
}


fn main() {
    let args: Args = Args::parse();

    match args.day {
        1 => match args.task {

        },
        _ => panic!()
    }
    println!("Hello, world!");
}
