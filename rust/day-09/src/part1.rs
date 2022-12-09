use std::collections::HashSet;

enum Move {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

fn add_pos(a: &Pos, b: &Pos) -> Pos {
    Pos {x: a.x+b.x, y: a.y + b.y}
}

fn sub_pos(a: &Pos, b: &Pos) -> Pos {
    Pos {x: a.x-b.x, y: a.y - b.y}
}

fn max_dist(a: &Pos, b: &Pos) -> i32 {
    if (a.x-b.x).abs() > (a.y-b.y).abs(){
       return (a.x-b.x).abs();
    }
    (a.y-b.y).abs()
}


fn move_rope(head: &mut Pos, tail: &mut Pos, direction: &Pos){
    
    let current_diff = sub_pos(&head, &tail);
    let mut new_pos = add_pos(&head, &direction);
    head.x = new_pos.x;
    head.y = new_pos.y;

    if max_dist(&head, &tail) >= 2 {
        new_pos = add_pos(&tail, &current_diff);
        tail.x = new_pos.x;
        tail.y = new_pos.y;
    } 
}

fn get_next_movment(line: &str) -> (Pos, i32) {
    let mut line_iter = line.split(" ");
    let dir_ch = line_iter.next().unwrap();
    let dir: Pos = if dir_ch == "R" {
        Pos{x: 1, y:0 }
    } else if dir_ch == "L" {
        Pos{x: -1, y:0 }
    } else if dir_ch == "U" {
        Pos{x: 0, y: 1 }
    } else {
        Pos{x: 0, y:-1 }
    };
    let num_steps = line_iter.next().unwrap().parse::<i32>().unwrap();
    return (dir, num_steps)
}


pub fn main(input: &str) -> String {
    let mut head = Pos {x:0, y:0};
    let mut tail = Pos {x:0, y:0};
    let mut direction;
    let mut num_steps;
    let mut visited = HashSet::new();
    visited.insert((tail.x, tail.y));
    for line in input.lines() {
        (direction, num_steps) = get_next_movment( &line);
        for i in 0..num_steps {
            move_rope(&mut head, &mut tail, &direction);
            visited.insert((tail.x, tail.y));
        }
    }
    visited.len().to_string()
}