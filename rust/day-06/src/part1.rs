use std::collections::HashMap;
use std::collections::VecDeque;


pub fn main(input: &str) -> String {
    let mut window = VecDeque::new();
    let mut h = HashMap::new();
    let mut char_iter = input.chars();
    let mut num_over_one: i32 = 0;
    let len: usize = 4;
    let mut c: char;    
    
    for _ in 0..len {
        c = char_iter.next().unwrap();
        window.push_back(c);
        h.entry(c).and_modify(|v| *v += 1).or_insert(1 as i32);
        if (*h.get(&c).unwrap() == 2) {
            num_over_one += 1;
        }
    }
    
    let mut i: usize = 4;
    while num_over_one != 0 {
        c = window.pop_front().unwrap();
        h.entry(c).and_modify(|v| *v -= 1);
        if (*h.get(&c).unwrap() == 1) {
            num_over_one -= 1;
        }

        c = char_iter.next().unwrap();
        window.push_back(c);
        
        h.entry(c).and_modify(|v| *v += 1).or_insert(1 as i32);
        if (*h.get(&c).unwrap() == 2) {
            num_over_one += 1;
        }
        i += 1;
    }
    i.to_string()
}