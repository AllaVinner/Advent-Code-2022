use std::fs;
use crate::day19c::{task1, task2};
mod day19c;

fn main() {
    let file = fs::read_to_string("./data/19_test.txt").unwrap().replace("\r", "");
    println!("Answer to part 1: {}", task1(&file));
    
    println!("Answer to part 2: {}", task2(&file));
}
