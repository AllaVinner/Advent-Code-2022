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
    let length = numbers.len()-1;
    for current_num in order.iter() {
        //println!("{:?}", numbers);
        let idx = numbers.iter().position(|v| v == current_num).unwrap() % (length+1);
        numbers.remove(idx);
        //let positive_value = (current_num  + numbers.len() as i64) as usize;
        let positive_value = ((current_num  % length as i64) + length as i64) as usize;
        numbers.insert(( idx +positive_value) % length, *current_num);
    }

    let zero_pos = numbers.iter().position(|v| *v == 0_i64).unwrap();
    numbers.rotate_left(zero_pos);
    let n = numbers.get(1000).unwrap() 
               + numbers.get(2000 ).unwrap() 
               + numbers.get(3000 ).unwrap(); 
    
    
    n.to_string()
}


pub fn task2(input: &str) -> String {
    
    "AAA".to_string()
}
