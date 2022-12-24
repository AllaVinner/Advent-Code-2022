use std::ops::{Add, Sub};


const WIDTH: i32 = 7;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}




impl Add<&Point> for & Point {
    type Output = Point;
    fn add(self, other: & Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<&Point> for & Point {
    type Output = Point;
    fn sub(self, other: & Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


fn get_shapes() -> Vec::<Vec::<Point>> {
    let mut v: Vec::<Vec::<Point>> = Vec::new();
    let mut vv: Vec::<Point>;
    
    // _ shape
    vv = Vec::new();
    vv.push(Point{x:0, y:0});
    vv.push(Point{x:1, y:0});
    vv.push(Point{x:2, y:0});
    vv.push(Point{x:3, y:0});
    v.push(vv);

    // + shape
    vv = Vec::new();
    vv.push(Point{x:1, y:0});
    vv.push(Point{x:1, y:1});
    vv.push(Point{x:0, y:1});
    vv.push(Point{x:2, y:1});
    vv.push(Point{x:1, y:2});
    v.push(vv);

    // L revser shape
    vv = Vec::new();
    vv.push(Point{x:0, y:0});
    vv.push(Point{x:1, y:0});
    vv.push(Point{x:2, y:0});
    vv.push(Point{x:2, y:1});
    vv.push(Point{x:2, y:2});
    v.push(vv);
    
    // | shape
    vv = Vec::new();
    vv.push(Point{x:0, y:0});
    vv.push(Point{x:0, y:1});
    vv.push(Point{x:0, y:2});
    vv.push(Point{x:0, y:3});
    v.push(vv);
    
    
    // square shape
    vv = Vec::new();
    vv.push(Point{x:0, y:0});
    vv.push(Point{x:1, y:0});
    vv.push(Point{x:1, y:1});
    vv.push(Point{x:0, y:1});
    v.push(vv);

    v
}

fn parse_char(c: char) -> i32 {
    match c {
        '>' => 1,
        '<' => -1,
        _ => panic!("Did not get > or <.")
    }
}

pub fn main(input: &str) -> String {
    let mut floor = Vec::<Point>::new();
    let mut top: u64 = 1;
    let shapes = get_shapes();
    let mut num_dropped: u64 = 0;
    let max_dropped: u64 = 2022;
    let will_break = false;
    
    println!("{:?}", parse_char(input.chars().next().unwrap()));
    for i in 0..WIDTH {
        floor.push(Point{x:i, y: 0});
    }
    loop {
        for direction in input.chars().map(|c| parse_char(c)) {
            move_side 
        }

        break;
        if(will_break) {
            break;
        }
    }
    

    
    "DONe".to_string()
}


