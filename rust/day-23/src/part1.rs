
use std::ops::{Add, Sub};
use std::ops::Mul;


#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: & Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, other: & Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}


fn parse_input_to_point_vec(input: &str) -> Vec<Point> {
    input.chars().filter_map(|c| match c {
        '#' => {
            x += 1;
            Some(Point{x:x, y:y})
        },
        '.' => {
           x += 1;
           None
        },
        '\n' => {
            y += 1;
            x = 0;
            None
        },
        _ => panic!(""),
    }).collect::<Vec<Point>>();
}



pub fn main(input: &str) -> String {
    let mut x = 0;
    let mut y = 1;
    let mut agents = parse_input_to_point_vec(input);
    
    dbg!(a);
    println!("{:?}", x);
    "Done".to_string()
}


