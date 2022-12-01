use day_01::process_part1;
use std::fs;

fn main() {
    let mut file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&mut file));
}