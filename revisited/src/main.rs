use std::fs;
mod day01;

fn main() {
    let file = fs::read_to_string("./data/01.txt").unwrap().replace("\r", "");
    println!("Answer to part 1: {}", day01::task1(&file));
}