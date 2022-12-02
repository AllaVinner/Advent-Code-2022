use std::fs;
mod part1;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap().replace("\r", "");
    println!("{}", part1::main(&file));
}