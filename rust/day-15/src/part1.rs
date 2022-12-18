use nom;
use nom::IResult;
use nom::sequence::preceded;
use nom::bytes::complete::tag;
use std::ops::Add;
use std::ops::Sub;

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

#[derive(Debug)]
struct Sensor {
    pos: Point,
    reach: i32,
}

fn distance(p1: &Point, p2: &Point) -> i32 {
    let p = p2 - p1;
    p.x.abs() + p.y.abs()
}

fn parse_sensors(input: &str) -> Vec::<Sensor> {
    let mut sensors = Vec::<Sensor>::new();
    for line in input.lines() {
        let (mut s, sx) = (preceded(tag("Sensor at x="), nom::character::complete::i32)(line) as IResult<&str, i32>).unwrap();
        let (mut s, sy) = (preceded(tag(", y="), nom::character::complete::i32)(s) as IResult<&str, i32>).unwrap();
        let (mut s, bx) = (preceded(tag(": closest beacon is at x="), nom::character::complete::i32)(s) as IResult<&str, i32>).unwrap();
        let (mut s, by) = (preceded(tag(", y="), nom::character::complete::i32)(s) as IResult<&str, i32>).unwrap();
        sensors.push(Sensor {
            pos : Point{x: sx, y: sy},
            reach: distance(&Point{x: sx, y: sy}, &Point{x: bx, y: by})
        });    
    }
    sensors
}



pub fn main(input: &str) -> String {
    let mut sensors = parse_sensors(input);
    println!("{:?}", sensors);
   
    "DOne".to_string()
}


