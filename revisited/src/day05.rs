use std::str::Chars;

struct CargoValue<'a> {
    iter: Chars<'a>
}

impl<'a> Iterator for CargoValue<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next();
        self.iter.next();
        self.iter.next();
        self.iter.next()
    }
}

trait CargoIterator {
    fn cargo_value(&self) -> CargoValue<'_>;
}

impl CargoIterator for &str {
    fn cargo_value(&self) -> CargoValue<'_> {
        CargoValue {iter: self.chars()}
    }
}


pub fn task1(input: &str) -> String {
    let (init_input, change_input) = input.split_once("\n\n").unwrap();
    
    init_input.lines()
        .rev()
        .skip(1)

        .step_by(4)
        .inspect(|c| println!("{:?}", c))
        .count();

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

