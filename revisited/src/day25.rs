

fn parse_digit(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Got none expected character.")
    }
}

fn as_base_5(n: u64) -> String {
    let max_pow = (n as f64).log(5 as f64).floor() as u32;
    let mut num_base_5 =  String::new();
    let mut rem = n;
    let mut div;
    for i in 0..=max_pow {
        div = rem / 5_u64.pow(max_pow-i);
        rem = rem % 5_u64.pow(max_pow-i);
        num_base_5.push_str(&div.to_string());
    }
    num_base_5
}

fn get_normalizing_constant(n: u64) -> u64 {
    let max_pow = (n as f64).log(5 as f64).floor() as u64;
    (0..=max_pow).fold(0, |acc, _| 2 + 5*acc)

} 

fn num_to_snafu(n: u64) -> String {
    let m = get_normalizing_constant(n);
    as_base_5(n+m).chars().map(|c| match c {
        '4' => '2',
        '3' => '1',
        '2' => '0',
        '1' => '-',
        '0' => '=',
        _ => panic!("In Num to snafu, got bad char")
    }).collect::<String>()
}



fn snafu_to_num(snafu: &str) -> u64 {
    snafu.chars().fold(0, |n, c| 5*n+parse_digit(c)) as u64
}

pub fn task1(input: &str) -> String {
    let n = input.lines()
        .map(|line| snafu_to_num(line))
        .sum::<u64>();
    num_to_snafu(n)
}


pub fn task2(input: &str) -> String {
    "AAA".to_string()
}



