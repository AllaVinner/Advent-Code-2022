use std::fs;
use crate::day22::{task1, task2};
mod day22;

fn main() {
    let file = fs::read_to_string("./data/22_test.txt").unwrap().replace("\r", "");
    println!("Answer to part 1: {}", task1(&file));
    println!("Answer to part 2: {}", task2(&file));
}