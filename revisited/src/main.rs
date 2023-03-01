use std::fs;
mod day02;

fn main() {
    let file = fs::read_to_string("./data/02.txt").unwrap().replace("\r", "");
    println!("Answer to part 1: {}", day02::task1(&file));
    println!("Answer to part 2: {}", day02::task2(&file));
}