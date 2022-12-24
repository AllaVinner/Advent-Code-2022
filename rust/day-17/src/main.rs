use std::fs;
mod part1;



fn main() {
    let file = fs::read_to_string("./test.txt").unwrap().replace("/r", "");
    println!("Answer part 1: {:?}", part1::main(&file));
}
