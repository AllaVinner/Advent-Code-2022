
use nom::IResult;
use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}


impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Point> for &Point {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}




pub fn main1(input: &str) -> String {
    let mut points = Vec::<[usize; 2]>::new();
    let mut iter;
    let mut x: usize;
    let mut y: usize;
    //input.lines().map(|line| line.split(" -> ").map(|pair| pair.split(",").map(|n| a.push(n.parse::<i32>().unwrap()))));
    
    for line in input.lines(){
        for (i, pair) in line.split(" -> ").enumerate() {
            iter = pair.split(",");
            x = iter.next().unwrap().parse::<usize>().unwrap();
            y = iter.next().unwrap().parse::<usize>().unwrap();
            
        }
    }
    
    "Done".to_string()
}



pub fn main(input: &str) -> String {
    let a = Point {x: 1, y: 1};
    let b = Point {x: 1, y: 1};
    let c = &a + &b;
    println!("{:?}", a + b);
    "Done".to_string()
}