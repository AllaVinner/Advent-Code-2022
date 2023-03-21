use std::ops::Add;
use std::collections::VecDeque;

const N: usize = 14;

pub fn benchmark_0(input: &str) -> String {
    input.chars()
        .skip(N)
        .enumerate()
        .scan(input.chars().take(N).collect::<VecDeque<char>>(),|state, (i, c)| {
            state.pop_front();
            state.push_back(c);
            if state.iter().enumerate().find(|(i, &c)| state.iter().skip(*i+1).find(|&c2| *c2==c).is_some()).is_some() {
                return Some(i);
            } else {
                return None;
            };
        })
        .last()
        .unwrap()
        .add(16)
        .to_string()
}
