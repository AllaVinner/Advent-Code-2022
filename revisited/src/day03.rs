





pub fn task1(input: &str) -> String {
    let a: Vec<char> = input.lines()
        .map(|line| (line, line.chars().count()))
        .map(|(line, len)| line.split_at(len / 2))
        .map(|(bag1, bag2)| bag1.chars().find(|c1| bag2.find(*c1).is_some()).unwrap())
        .take(5)
        .collect();
    println!("{:?}", a);
    "asdf".to_string()
}


pub fn task2(input: &str) -> String {
    
    "asdf".to_string()
}


