use std::collections::VecDeque;


fn parse_input(input: &str) -> (VecDeque<i64>, VecDeque<i64>) {
    let mut numbers = VecDeque::<i64>::new();
    let mut order = VecDeque::<i64>::new();
    for line in input.lines() {
        numbers.push_back(line.parse().unwrap());
        order.push_back(line.parse().unwrap());
    }
    (numbers, order)
}

pub fn task1(input: &str) -> String {
    let (mut numbers, mut order) = parse_input(input);
    println!("{:?}", numbers);
    println!("{:?}", order);
    "AAA".to_string()
}



pub fn task2(input: &str) -> String {
    
    "AAA".to_string()
}
