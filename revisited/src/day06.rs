
use std::ops::Add;
use itertools::Itertools;
use std::collections::VecDeque;

struct Message {
    
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
            if state.contains(&c) {
                return None;
            } else {
                state.pop_back();
                state.push_front(c);
                return Some(i);
            };
        })
        .sum::<usize>()
        .to_string()
}
