use std::collections::HashSet;

const LEN: usize = 10;

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn normalize(d: &mut [i32; 2] ) {
    for i in 0..2 {
        if d[i] < -1 {
            d[i] = -1;
        } 
        if d[i] > 1 {
            d[i] = 1;
        } 
    }
}

fn move_rope(rope: &mut [[i32; 2]; LEN], direction: &[i32; 2]){
    let mut dir: [i32; 2] = *direction;

    for i in 0..(LEN-1) {
        // Move head
        rope[i][0] = rope[i][0] + dir[0];
        rope[i][1] = rope[i][1] + dir[1];

        dir = [rope[i][0] - rope[i+1][0],
               rope[i][1] - rope[i+1][1]];
        if dir[0].abs() >= 2  || dir[1].abs() >= 2 {
            normalize(&mut dir);
        } else {
            dir = [0, 0];
        }
    }

    rope[LEN-1][0] = rope[LEN-1][0] + dir[0];
    rope[LEN-1][1] = rope[LEN-1][1] + dir[1];
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
        (direction, num_steps) = get_next_movment( &line);
        for i in 0..num_steps {
            move_rope(&mut rope, &direction);
            visited.insert(rope[LEN-1]);
        }
    }
    
    visited.len().to_string()
}

