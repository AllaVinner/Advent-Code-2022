use std::fs;
mod part1;
//mod part2;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap().replace("\r", "");
    println!("Answer to part 1: {}", part1::main(&file));
    //println!("Answer to part 2: {}", part2::main(&file));

}

