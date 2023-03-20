

fn parse_digit(c: char) -> isize {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Got none expected character.")
    }
}

pub fn task1(input: &str) -> String {
    input.lines()
        .map(|line| line.chars().fold(0, |n, c| 5*(n+parse_digit(c)) ))
        .sum::<isize>()
        .to_string()
}


pub fn task2(input: &str) -> String {
    "AAA".to_string()
}



