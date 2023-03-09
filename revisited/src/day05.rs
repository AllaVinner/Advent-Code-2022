use std::{str::Chars, collections::HashMap};

struct CargoIter<'a> {
    iter: Chars<'a>
}



impl<'a> Iterator for CargoIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next();
        self.iter.next();
        self.iter.next();
        self.iter.next()    
    }
}

trait CargoIterator {
    fn cargo_value(&self) -> CargoIter<'_>;
}

impl CargoIterator for &str {
    fn cargo_value(&self) -> CargoIter<'_> {
        CargoIter {iter: self.chars()}
    }
}

struct Cargo {
    cargo: HashMap<char, Vec<char>>
}

fn init_parser(input: &str) {
    let mut cargo: HashMap< > =input.lines().rev().next().unwrap()
        .chars().enumerate().filter(|(i, c)| *c != ' ')
        .map(|(i, c)| ).collect()
}


pub fn task1(input: &str) -> String {
    let (init_input, change_input) = input.split_once("\n\n").unwrap();
    
    let mut a = vec![1,2,3];
    a.push(5);
    println!("{:?}", &a);
    a.pop();
    println!("{:?}", &a);
    "ASD".to_string()
}


pub fn task2(input: &str) -> String {
    "ASD".to_string()
}

