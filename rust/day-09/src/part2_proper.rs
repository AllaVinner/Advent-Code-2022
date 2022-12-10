use std::collections::HashSet;
use std::ops::Add;

const LEN: usize = 10;
const DIM: usize = 2;

struct Point {
    x: [i32; DIM]
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        let new = Point {x: [0; DIM]};
        for i in 0..DIM {
            new.x[i] = self.x[i] + other.x[i]; 
        }
        new
    }
}


fn normalize(p: &mut Point) {
    for i in 0..DIM {
        if p.x[i] < -1 {
            p.x[i] = -1;
        } 
        if p.x[i] > 1 {
            p.x[i] = 1;
        } 
    }
}

fn move_rope(rope: &mut [Point; LEN], direction: &Point) {
    let mut dir = *direction;

    for i in 0..(LEN-1) {
        // Move head
        *rope[i] = *rope[i] + *dir;

        dir = [rope[i].x[0] - rope[i+1].x[0],
               rope[i].x[1] - rope[i+1].x[1]];
        if dir[0].abs() >= 2  || dir[1].abs() >= 2 {
            normalize(&mut dir);
        } else {
            dir = [0, 0];
        }
    }

    rope[LEN-1].x[0] = rope[LEN-1].x[0] + dir.x[0];
    rope[LEN-1].x[1] = rope[LEN-1].x[1] + dir.x[1];
}

fn get_next_movment(line: &str) -> ([i32; 2], i32) {
    let mut line_iter = line.split(" ");
    let dir = match line_iter.next().unwrap() {
        "R" => [1, 0],
        "L" => [-1, 0],
        "U" => [0, 1],
        "D" => [0, -1],
        _ => panic!("Got direction not in RLUD"),
    };
    let num_steps = line_iter.next().unwrap().parse::<i32>().unwrap();
    return (dir, num_steps)
}

pub fn main(input: &str) -> String {
    let mut rope = [[0, 0]; LEN];
    let mut direction: [i32; 2];
    let mut num_steps: i32;
    let mut visited = HashSet::new();

    visited.insert(rope[LEN-1]);
    for line in input.lines() {
        (direction, num_steps) = get_next_movment(&line);
        for _ in 0..num_steps {
            move_rope(&mut rope, &direction);
            visited.insert(rope[LEN-1]);
        }
    }
    
    visited.len().to_string()
}
