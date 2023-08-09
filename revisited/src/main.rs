use std::fs;
use crate::day24a::{task1, task2};
mod day24a;

fn main() {
    let file = fs::read_to_string("./data/24.txt").unwrap().replace("\r", "");
    println!("Answer to part 1: {}", task1(&file));
    
    println!("Answer to part 2: {}", task2(&file));
}