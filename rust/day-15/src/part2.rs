

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

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
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

fn parse_sensors_and_beacons(input: &str) -> (Vec::<Sensor>, Vec::<Point>) {
    let mut sensors = Vec::<Sensor>::new();
    let mut beacons = Vec::<Point>::new();
    for line in input.lines() {
        let (mut s, sx) = (preceded(tag("Sensor at x="), nom::character::complete::i32)(line) as IResult<&str, i32>).unwrap();
        let (mut s, sy) = (preceded(tag(", y="), nom::character::complete::i32)(s) as IResult<&str, i32>).unwrap();
        let (mut s, bx) = (preceded(tag(": closest beacon is at x="), nom::character::complete::i32)(s) as IResult<&str, i32>).unwrap();
        let (mut s, by) = (preceded(tag(", y="), nom::character::complete::i32)(s) as IResult<&str, i32>).unwrap();
        sensors.push(Sensor {
            pos : Point{x: sx, y: sy},
            reach: distance(&Point{x: sx, y: sy}, &Point{x: bx, y: by})
        });  
        beacons.push(Point {x: bx, y: by});  
    }
    (sensors, beacons)
}

fn is_reached(pos: &Point, sensors: &Vec::<Sensor>) -> bool {
    for sensor in sensors.iter() {
        if sensor.reach >= distance(&sensor.pos, pos) {
            return true;
        }
    }
    false
}


fn get_surrounding(sensor: &Sensor) -> Vec::<Point> {
    let mut vec = Vec::<Point>::new();
    for rx in 0..(sensor.reach+1) {
        vec.push(Point{x: sensor.pos.x+rx, y: sensor.pos.y + (sensor.reach+1-rx)});
        vec.push(Point{x: sensor.pos.x+rx, y: sensor.pos.y - (sensor.reach+1-rx)});
        vec.push(Point{x: sensor.pos.x-rx, y: sensor.pos.y + (sensor.reach+1-rx)});
        vec.push(Point{x: sensor.pos.x-rx, y: sensor.pos.y - (sensor.reach+1-rx)});
    }
    vec.push(Point{x: sensor.pos.x+sensor.reach, y: sensor.pos.y});
    vec
}


pub fn main(input: &str) -> String {
    let (mut sensors, mut beacons) = parse_sensors_and_beacons(input); 
    //let max_box = 20;
    let max_box = 4_000_000;
    let mut surrounding: Vec::<Point> = Vec::<Point>::new();
    let mut siter = sensors.iter();

    for sensor in sensors.iter() {
        surrounding = get_surrounding(&sensor);
        surrounding.retain(|p| p.x >=0 && p.y >=0 && p.x <= max_box && p.y <= max_box);
        surrounding.retain(|p| !is_reached(p, &sensors));
        if surrounding.len() == 1 {
            break;
        } 
        if surrounding.len() > 1 {
            panic!("More than one free?")
        } 
    }
    let point = surrounding.pop().unwrap();
    println!("{:?}", point);
    (4_000_000*(point.x as i64) +point.y as i64).to_string()
}


/*
pub fn main1(input: &str) -> String {
    let (mut sensors, mut beacons) = parse_sensors_and_beacons(input); 
    for sensor in sensors.iter() {
        surrounding = get_surrounding(&sensor);
        surrounding = filter_outside_box(&surrounding);
        surrounding = filter_not_reached(surrounding, sensors);
        if surrounding.len() == 1 {
            break;
        } 
        if surrounding.len() > 1 {
            panic!("More than one free?")
        } 
        
    }   
    counter.to_string()
}

*/

