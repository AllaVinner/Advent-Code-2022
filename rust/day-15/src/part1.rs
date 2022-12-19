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

fn total_reach_along_x(ycoord: i32, sensors: &Vec::<Sensor>) -> (i32, i32) {
    let mut min_x = 0;
    let mut max_x = 0;
    for sensor in sensors.iter() {
        let dist_to_line = (sensor.pos.y - ycoord).abs();
        if dist_to_line >= sensor.reach {
            continue;
        }
        if sensor.pos.x + (sensor.reach - dist_to_line) > max_x {
            max_x = sensor.pos.x + (sensor.reach - dist_to_line);
        }
        
        if sensor.pos.x - (sensor.reach - dist_to_line) < min_x {
            min_x = sensor.pos.x - (sensor.reach - dist_to_line);
        }

    }
    (min_x, max_x)
}

pub fn main(input: &str) -> String {
    let (mut sensors, mut beacons) = parse_sensors_and_beacons(input);
    
    let y: i32 = 2000000;
    //let y: i32 = 10;
    let (min_x, max_x) = total_reach_along_x(y, &sensors);
    let mut counter = 0;
    println!("Is reachced {:?}", is_reached(&Point{x:-22288355, y:10}, &sensors));
    println!("min {:?}", min_x);
    println!("max {:?}", max_x);
    for i in min_x..(max_x+1) {
        match beacons.iter().find(|p| *p == &Point{x: i, y: y}) {
            Some(p) => continue,
            None => (),
        };

        if !is_reached(&Point{x:i, y:y}, &sensors) {
            continue;
        }
        counter += 1;
        
    }


   
    counter.to_string()
}


