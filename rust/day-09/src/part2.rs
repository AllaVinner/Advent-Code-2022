use std::collections::HashSet;


const LEN: usize = 10;

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn is_diag(d: & [i32; 2]) -> bool {
    if d[0].abs() > 0 && d[1].abs() > 0 {
        return true;
    }
    false
}

fn move_rope(rope: &mut [[i32; 2]; LEN], direction: &[i32; 2]){
    let mut current_pos: [i32; 2];
    let mut prev_dir: [i32; 2];
    let mut current_dir: [i32; 2] = [direction[0], direction[1]];
    let mut first_diag: bool;
    let mut sec_diag: bool;

    for i in 0..(LEN-1) {
        current_pos = [rope[i][0], rope[i][1]];
        prev_dir = [rope[i][0]-rope[i+1][0], rope[i][1]-rope[i+1][1]];
        // Move head
        rope[i][0] = rope[i][0] + current_dir[0];
        rope[i][1] = rope[i][1] + current_dir[1];

        first_diag = is_diag(&[current_pos[0]-rope[i+1][0],current_pos[1]-rope[i+1][1]]);
        sec_diag = is_diag(&[rope[i][0]-current_pos[0],rope[i][1]-current_pos[1]]);
        // should tail move?
        if (rope[i][0]-rope[i+1][0]).abs() >= 2  || (rope[i][1]-rope[i+1][1]).abs() >= 2 {

            if first_diag && sec_diag {
                println!("Both diag");
                println!("{:?}", [rope[i][0]-current_pos[0],rope[i][1]-current_pos[1]]);
                current_dir = [(rope[i][0]- rope[i+1][0])/2, (rope[i][1]- rope[i+1][1])/2];
            } else if first_diag {
                println!("First diag");
                current_dir = [current_pos[0]-rope[i+1][0],current_pos[1]-rope[i+1][1]];
            } else if sec_diag {
                println!("Sec diag");
                current_dir = [rope[i][0]-current_pos[0],rope[i][1]-current_pos[1]];
            } else {
                current_dir = [current_pos[0]-rope[i+1][0],current_pos[1]-rope[i+1][1]];
            }
        } else {
            current_dir = [0, 0];
        }
    }

    rope[LEN-1][0] = rope[LEN-1][0] + current_dir[0];
    rope[LEN-1][1] = rope[LEN-1][1] + current_dir[1];
}

fn get_next_movment(line: &str) -> ([i32; 2], i32) {
    let mut line_iter = line.split(" ");
    let dir_ch = line_iter.next().unwrap();
    let dir: [i32; 2] = if dir_ch == "R" {
        [1, 0]
    } else if dir_ch == "L" {
        [-1, 0]
    } else if dir_ch == "U" {
        [0, 1]
    } else {
        [0, -1]
    };
    let num_steps = line_iter.next().unwrap().parse::<i32>().unwrap();
    return (dir, num_steps)
}

pub fn main(input: &str) -> String {
    let mut rope = [[0, 0]; LEN];
    let mut direction: [i32; 2];
    let mut num_steps: i32;
    let mut visited = HashSet::new();

    visited.insert((rope[LEN-1][0], rope[LEN-1][1]));
    for line in input.lines() {
        (direction, num_steps) = get_next_movment( &line);
        println!("..............");
        println!("{:?}", rope);
        for i in 0..num_steps {
            move_rope(&mut rope, &direction);
            visited.insert((rope[LEN-1][0], rope[LEN-1][1]));
            println!(     "{:?}", rope);
        }
        
    }
    

    visited.len().to_string()
}

