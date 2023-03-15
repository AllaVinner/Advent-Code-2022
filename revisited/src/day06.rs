
use std::ops::Add;
use itertools::Itertools;
use std::collections::VecDeque;

const N: usize = 14;
struct Message {
    registry: [usize; N],
    occupied: [usize; 128],
    start_index: usize
}



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
    input.chars()
        .skip(14)
        .enumerate()
        .scan(input.chars().take(14).collect::<VecDeque<char>>(),|state, (i, c)| {
            state.pop_front();
            state.push_back(c);
            if state.iter().enumerate().find(|(i, &c)| state.iter().skip(*i+1).find(|&c2| *c2==c).is_some()).is_some() {
                return Some(i);
            } else {
                println!("{:?}", &state);
                return None;
            };
        })
        .last()
        .unwrap()
        .add(16)
        .to_string()
}
