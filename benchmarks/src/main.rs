use std::fs;
use clap::{Parser};
use clap;
use std::string::String;
use std::time::Instant;

mod day06_2;

#[derive(Parser, Debug)]
#[command(author, version, 
    about="THis is a great command", 
    long_about = "Actually This is the greatest that ever existed")]
struct Args {
    #[arg(short, long, default_value_t=6)]
    day: u32,

    #[arg(short, long, default_value_t=1)]
    task: u32,

    #[arg(short, long, default_value_t=0)]
    benchmark_id: u32,
    
    #[arg(short, long)]
    input_file: String,

    #[arg(short, long, default_value_t=10)]
    num_iterations: u32,
}




fn main() {
    let args: Args = Args::parse();
    let input = fs::read_to_string(args.input_file).unwrap().replace("\r", "");

    let fun: fn(&str) -> String = match args.day {
        6 => match args.task {
            2 => match args.benchmark_id {
                0 => day06_2::benchmark_0,
                1 => day06_2::benchmark_1,
                _ => panic!("Benchmark {} for task {} on day {} is not implemented.",args.benchmark_id, args.task, args.day),
            },
            _ => panic!("Task {} on day {} is not implemented.", args.task, args.day),
        },
        _ => panic!("Day {} is not implemented.", args.day),
    };
    let mut ans: String = "".to_string();
    let now = Instant::now();
    for _ in 0..args.num_iterations{
        ans = fun(&input);
    }
    let elapsed = now.elapsed() / args.num_iterations;
    println!("Time: {:.2?},  with answere {}, of benchmark {} for task {} on day {}", elapsed, ans, args.benchmark_id, args.task, args.day);
} 
