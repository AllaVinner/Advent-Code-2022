
use std::ops::Add;
use itertools::Itertools;


pub fn task1(input: &str) -> String {
    input.chars()
        .tuple_windows()
        .enumerate()
        .find(|(i, (c0,c1,c2,c3))| c0 != c1 && c0 != c2 && c0 != c3 && c1 != c2 && c1 != c3 && c2 != c3)
        .unwrap()
        .0
        .add(4)
        .to_string()
}


pub fn task2(input: &str) -> String {
    "ASD".to_string()
}
