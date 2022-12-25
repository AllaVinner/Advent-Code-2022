use std::ops::{Add, Sub};
use std::collections::HashSet;

const WIDTH: i32 = 7;

#[derive(Debug, Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Eq for Point {}

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

fn side_move(direction: i32, pos: &mut Point, shape: &Vec::<Point>, floor: &HashSet::<Point>) {
    let mut p = Point{x:0, y:0};
    for shape_p in shape{
        p.x =  pos.x + shape_p.x + direction; 
        p.y = pos.y + shape_p.y; 
        if p.x < 0 || WIDTH <= p.x {
            return;
        }
        for floor_p in floor {
            if p == *floor_p {
                return;
            }
        }
    }
    pos.x = pos.x + direction;
}

fn down_move(pos: &mut Point, shape: &Vec::<Point>, floor: &HashSet::<Point>) -> bool {
    let mut p = Point{x:0, y:0};
    for shape_p in shape{
        p.x =  pos.x + shape_p.x; 
        p.y = pos.y + shape_p.y-1; 
        for floor_p in floor {
            if p == *floor_p {
                return true;
            }
        }
    }
    pos.y = pos.y -1;
    false
}


fn get_neighbors(point: Point) -> Vec::<Point> {
    let mut points = Vec::new();
    if point.y < 4 {
        points.push(Point{x:point.x,y:point.y+1});
    }
    if point.x < WIDTH-1 {
        points.push(Point{x:point.x+1, y:point.y});
    }
    if point.x > 0 {
        points.push(Point{x:point.x-1, y:point.y});
    }
    points.push(Point{x:point.x, y:point.y-1});
    points
}


fn update_floor(floor: &HashSet::<Point>) -> HashSet::<Point> {
    let mut new_floor = HashSet::<Point>::new();
    let mut visited = HashSet::<Point>::new();
    let mut stack = Vec::<Point>::new();
    let mut tmp_pos;
    let mut neighbors;
    stack.push(Point{x:2, y:4});
    visited.insert(Point{x:2, y:4});
    while !stack.is_empty() {
        tmp_pos = stack.pop().unwrap();
        visited.insert(tmp_pos);
        neighbors = get_neighbors(tmp_pos);
        for n in neighbors {
            if floor.contains(&n) {
                if !new_floor.contains(&n){
                    new_floor.insert(n);
                }
                continue;
            }
            if !visited.contains(&n){
                stack.push(n);
            }
        }
        
    }
    new_floor
}

fn add_shape_to_floor(pos: &Point, shape: &Vec::<Point>, floor: &mut HashSet::<Point>) {
    for s in shape {
        floor.insert(s + pos);
    }
}

pub fn main(input: &str) -> String {
    let mut floor = HashSet::new();
    let mut top: u64 = 1;
    let shapes = get_shapes();
    let mut num_dropped: u64 = 0;
    
    let mut shape = shapes.get(num_dropped as usize % shapes.len()).unwrap();
    let max_dropped: u64 = 2022; //2022;
    let mut will_break = false;
    let mut pos = Point{x:2, y:4 };
    let mut crashed;
    // Set init floor
    for i in 0..WIDTH {
        floor.insert(Point{x:i, y: 0});
    }
    let mut i = 0;
    loop {
        for direction in input.chars().map(|c| parse_char(c)) {
            if i as u64 > 1_000_000_000_000 {
                println!("DDDIIIOONE{:?}", pos);
                will_break = true;
                break;
            }
            i += 1;
            side_move(direction, &mut pos, shape, &floor);
            crashed = down_move(&mut pos, shape, &floor);
            if crashed {
                add_shape_to_floor(&pos, shape, &mut floor);
                floor = update_floor(&floor);
                num_dropped += 1;
                shape = shapes.get(num_dropped as usize % shapes.len()).unwrap();
                println!("{:?}", floor);
                if num_dropped == max_dropped {
                    will_break = true;
                    break;
                }
            }
        }

        if(will_break) {
            break;
        }
    }

    
    "DONe".to_string()
}


