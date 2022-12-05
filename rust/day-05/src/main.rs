use std::fs;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    sequence::delimiter,
};

//mod part1;
//mod part2;
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}



fn main() {
    //let file = fs::read_to_string("./input.txt").unwrap().replace("\r", "");
    //println!("Answer to part 1: {}", part1::main(&file));
    //println!("Answer to part 2: {}", part2::main(&file));
    let a = String::from("abc");
    let b = &a;
    let c: nom::IResult<&str, char> = alt((
        tag("   "),
        delimiter(complete::char('['), 
        complete::char('b'))
    )("bcbb");
    println!("Type of {:?}", type_of(&c));
    println!("Val {:?}", c);
}
