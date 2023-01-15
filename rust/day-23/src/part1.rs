use core::cmp::max;
use std::ops::{Add, Sub};
use std::ops::Mul;
use std::collections::VecDeque;

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

enum Directions {
    North,
    South,
    West,
    East
}

fn len(p: &Point) -> i32 {
    max(p.x.abs(), p.y.abs())
}

fn parse_input_to_point_vec(input: &str) -> Vec<Point> {
    let mut x: i32 = 0;
    let mut y: i32 = 1;
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
    }).collect::<Vec<Point>>()
}

fn is_neightbours(a: &Point, b: &Point) -> bool {
    len(&(a-b)) <= 1
}

fn get_neightbours(index: usize, agents: &Vec<Point>) -> Vec<usize> {
    agents.iter().enumerate().filter_map(|(i, p)| {
        if i == index {
            return None;
        }
        if is_neightbours(agents.get(index).unwrap(), agents.get(i).unwrap()) {
            return Some(i)
        }
        return None
    }).collect()
}

fn is_direction_free(index: usize, direction: Directions, agents: &Vec<Point>) -> bool {
    let a = agents.get(index).unwrap();
    let points: Vec<Point> = match direction {
        Directions::North => vec![Point{x:-1,y:-1},Point{x:0,y:-1},Point{x:1,y:-1}],
        Directions::South => vec![Point{x:-1,y:1},Point{x:0,y:1},Point{x:1,y:1}],
        Directions::West => vec![Point{x:-1,y:-1},Point{x:-1,y:0},Point{x:-1,y:1}],
        Directions::East => vec![Point{x:1,y:-1},Point{x:1,y:0},Point{x:1,y:1}]
    }
    agents.iter().filter(|a1| points.find(|p| &(a+p)==*a1)).len() == 0
}

fn get_proposal(index: usize, directions: VecDeque<Directions>, agents: &Vec<Point>) -> Optiona(Point) {
    let neightbours = get_neightbours(index, agents);
    if len(neightbours) == 0{
        return None;
    }
    for direction in directions {
        if is_direction_free(index, direction, agents) {
            //return Some(agents.get(index).unwrap() + direction);
            return None
        }
    }
    None
}


pub fn main(input: &str) -> String {
    let mut agents = parse_input_to_point_vec(input);

    println!("{:?}", get_neightbours(0, &agents));
    println!("{:?}", is_direction_free(0, Directions::North, &agents));
    "Done".to_string()
}


