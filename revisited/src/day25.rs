

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

fn as_base_5(n: u64) -> String {
    let max_pow = (n as f64).log(5 as f64).floor() as u32;
    let mut num_base_5 =  String::new();
    let mut rem = n ;
    let mut div;
    for i in 0..=max_pow {
        div = rem / 5_u64.pow(max_pow-i) as u64;
        rem = rem % 5_u64.pow(max_pow-i) as u64;
        num_base_5.push_str(&div.to_string());
    }
    num_base_5
}

fn get_normalizing_constant(n: u64) -> u64 {
    let max_pow = (n as f64).log(5 as f64).floor() as u64;
    let mut N = 0;
    for _ in 0..=max_pow {
        N = 2 + 5*N;
    }
    N
} 

fn num_to_snafu(n: u64) -> String {
    let N = get_normalizing_constant(n);
    as_base_5(n+N).chars().map(|c| match c {
        '4' => '2',
        '3' => '1',
        '2' => '0',
        '1' => '-',
        '0' => '=',
        _ => panic!("In Num to snafu, got bad char")
    }).collect::<String>()
}



fn snafu_to_num(snafu: &str) -> isize {
    snafu.chars().fold(0, |n, c| 5*n+parse_digit(c))
}

pub fn task1(input: &str) -> String {
    let n = input.lines()
        .map(|line| snafu_to_num(line))
        .sum::<isize>();
    num_to_snafu(u64::try_from(n).ok().unwrap())
}


pub fn task2(input: &str) -> String {
    "AAA".to_string()
}



