use std::collections::HashMap;
use std::collections::VecDeque;


pub fn main(input: &str) -> String {
    let mut window = VecDeque::new();
    let mut h = HashMap::new();
    let mut char_iter = input.chars();
    let mut num_over_one: i32 = 0;
    let len: usize = 4;

    let mut old_c: char;
    let mut c: char;    
    for ii in 0..4 {
        c = char_iter.next().unwrap();
        window.push_back(c);
        h.entry(c).and_modify(|v| *v += 1).or_insert(1 as i32);
        if (*h.get(&c).unwrap() == 2) {
            num_over_one += 1;
        }
    }
    
    let mut new_c: char;
    let mut i: usize = 4;
    while num_over_one != 0 {
        old_c = window.pop_front().unwrap();
        h.entry(old_c).and_modify(|v| *v -= 1);
        if (*h.get(&old_c).unwrap() == 1) {
            num_over_one -= 1;
        }

        new_c = char_iter.next().unwrap();
        window.push_back(new_c);
        
        h.entry(new_c).and_modify(|v| *v += 1).or_insert(1 as i32);
        if (*h.get(&new_c).unwrap() == 2) {
            num_over_one += 1;
        }

        i += 1;
    }
    println!("{:?}", i);
    

    //  map.get(&'a').cloned().unwrap_or(0);
    "AAA".to_string()
}