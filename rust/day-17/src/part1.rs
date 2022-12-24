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





fn update_floor(pos: &mut Point, shape: &Vec::<Point>, floor: &HashSet::<Point>) {
    let mut new_floor = HashSet::<Point>::new();
    let mut stack = Vec::<Point>::new();
    let tmp_pos;
    stack.push(Poins{x:2, y:4})
    while !stack.is_empty() {
        tmp_pos = stack.pop().unwrap();

        match floor.get(tmp_pos) {
            Some(p) => continue,
            None => ()
        };
        match new_floor.get(tmp_pos) {
            Some(p) => continue,
            None => ()
        };
        
    }
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
    let max_dropped: u64 = 2022;
    let will_break = false;
    let mut pos = Point{x:2, y:4 };
    let crashed;

    // Set init floor
    for i in 0..WIDTH {
        floor.insert(Point{x:i, y: 0});
    }
    let direction = parse_char(input.chars().next().unwrap());
    side_move(direction, &mut pos, shape, &floor);
    crashed = down_move( &mut pos, shape, &floor);
    println!("{:?}", crashed);
    add_shape_to_floor(&pos, shape, &mut floor);
    floor = update_floor(&floor);

/*
    loop {
        for direction in input.chars().map(|c| parse_char(c)) {
            //pos = side_move(&pos, &shape, &floor);
            //(pos, found_buttom) = down_move(&pos, &shape, &floor);
            if found_buttom {
                floor = update_floor(&floor, shape, pos);
                num_dropped += 1;
                shape = shapes.get(num_dropped as usize % shapes.len()).unwrap();
                if num_dropped == max_dropped {
                    will_break = true;
                    break;
                }
            }
        }
        break;
        if(will_break) {
            break;
        }
    }

*/
    

    
    "DONe".to_string()
}


