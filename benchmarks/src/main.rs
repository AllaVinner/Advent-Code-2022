use std::fs;
use clap::{Parser};
use clap;
use std::string::String;
use std::time::Instant;

mod day06_2;

#[derive(Parser, Debug)]
#[command(author, version, 
    about="Benchmarking different solutions of AoC 2022.")]
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
                2 => day06_2::benchmark_2,
                3 => day06_2::benchmark_3,
                4 => day06_2::benchmark_4,
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

    println!("Took {time:.2?} to produce answere {ans} of benchmark {benchmark_id} for task {task} on day {day}.", 
        time = elapsed, 
        ans = ans, 
        benchmark_id = args.benchmark_id, 
        task = args.task, 
        day = args.day);
} 

