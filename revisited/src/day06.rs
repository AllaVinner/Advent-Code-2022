

use itertools::Itertools;

pub fn task1(input: &str) -> String {
    let a: Vec<char> = input.chars().tuple_windows().map(|(c0,c1,c2,c3)| c2).collect();
    println!("{:?}", a);
    "ASD".to_string()
}


pub fn task2(input: &str) -> String {
    "ASD".to_string()
}
