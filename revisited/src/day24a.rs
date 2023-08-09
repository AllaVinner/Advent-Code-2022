use itertools::Itertools;
use std::fmt::{self, Display, Formatter};
use std::ops::Add;
use num::integer::lcm;


#[derive(Debug, Clone, Copy)]
enum Cell {
    EMPTY,
    NORTH,
    EAST,
    SOUTH,
    WEST
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SimpleCell {
    EMPTY,
    FULL
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", match self {
            Cell::EMPTY => '.',
            Cell::NORTH => '^',
            Cell::EAST => '>',
            Cell::SOUTH => 'v',
            Cell::WEST => '<'
        })
    }
}

impl Display for SimpleCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", match self {
            SimpleCell::EMPTY => '.',
            SimpleCell::FULL => 'X'
        })
    }
}

fn get_input_size(s: &str) -> (usize, usize) {
    let width = s.lines().take(1).last().unwrap().chars().count()-2;
    let height: usize = s.lines().count()-2;
    return (height, width);
}


fn char_to_cell(c: char) -> Cell {
    match c {
        '.' => Cell::EMPTY,
        '^' => Cell::NORTH,
        '>' => Cell::EAST,
        'v' => Cell::SOUTH,
        '<' => Cell::WEST,
        _ => panic!("Got unexpected character")
    }
}

fn parse_input(s: &str, height: usize, width: usize) -> Vec<Vec<Cell>> {
    s.lines().skip(1).take(height)
        .map(|line| {
            line.chars().skip(1).take(width)
                .map(|c|char_to_cell(c))
                .collect::<Vec<Cell>>()
        })
        .collect()
}



fn follow_storm(block: &mut Vec<Vec<Vec<SimpleCell>>>,
                row: usize, 
                col: usize,
                cell: Cell
                ) {
    let mut pos = Pos {x: col as i32, y: row as i32};
    let dir = match cell {
        Cell::EMPTY => return,
        Cell::NORTH => Pos{x: 0, y: -1},
        Cell::EAST => Pos{x: 1, y: 0},
        Cell::SOUTH => Pos{x: 0, y: 1},
        Cell::WEST => Pos{x: -1, y: 0}
    };

    let num_iter = block.len();
    let height = block[0].len();
    let width = block[0][0].len();
    for i in 0..num_iter {
        block[i][pos.y as usize][pos.x as usize] = SimpleCell::FULL;
        pos = pos + dir;
        pos.x = pos.x.rem_euclid(width as i32);
        pos.y = pos.y.rem_euclid(height as i32);
    }
}


fn generate_block(map: &Vec<Vec<Cell>>) -> Vec<Vec<Vec<SimpleCell>>> {
    let height = map.len();
    let width = map[0].len();

    let mut new_map = vec![vec![vec![SimpleCell::EMPTY; width]; height]; lcm(height, width)];

    // Find the storms
    for (row_i, row) in map.iter().enumerate() {
        for (col_i, cell) in row.iter().enumerate() {
            follow_storm(&mut new_map, row_i, col_i, *cell)
        }
    }

    // Travers each storm

    new_map
}



fn print_time_slice(map: &Vec<Vec<Cell>>) {
    for v in map.iter() {
        for c in v.iter() {
            print!("{} ", c);
        }
        println!(" ");
    }
}

fn print_time_block(map: &Vec<Vec<Vec<SimpleCell>>>) {
    for (t, time) in map.iter().enumerate() {
        println!(" ");
        println!(":{} ------", t);
        for v in time.iter() {
            for c in v.iter() {
                print!("{} ", c);
            }
            println!(" ");
        }
        println!(" ");
    }
}

fn in_bounds(row: i32, col: i32, height: usize, width: usize) -> bool {
    if row < 0 || row >= height as i32 {
        return false;
    }
    if col < 0 || col >= width as i32 {
        return false;
    }
    return true;
}

fn travers(block: &Vec<Vec<Vec<SimpleCell>>>) -> String {
    
    // Get possible times to start 
    let height = block[0].len();
    let width = block[0][0].len();


    // create current time sheet
    let mut current_map = vec![vec![SimpleCell::EMPTY; width]; height];
    let mut next_map = vec![vec![SimpleCell::EMPTY; width]; height];
    let mut i = 0;
    let mut counter = 0;
    loop {
        for ri in 0..height {
            for ci in 0..width {
                if current_map[ri][ci] == SimpleCell::EMPTY {
                    continue;
                }
                if block[i][ri][ci] == SimpleCell::EMPTY {
                    next_map[ri][ci] = SimpleCell::FULL;
                }
                if in_bounds(ri as i32 +1, ci as i32, height, width) {
                    if block[i][ri+1][ci] == SimpleCell::EMPTY {
                        next_map[ri+1][ci] = SimpleCell::FULL;
                    }
                }
                if in_bounds(ri as i32 -1, ci as i32, height, width) {
                    if block[i][ri-1][ci] == SimpleCell::EMPTY {
                        next_map[ri-1][ci] = SimpleCell::FULL;
                    }
                }
                if in_bounds(ri as i32, ci as i32 + 1, height, width) {
                    if block[i][ri][ci+1] == SimpleCell::EMPTY {
                        next_map[ri][ci+1] = SimpleCell::FULL;
                    }
                }
                if in_bounds(ri as i32, ci as i32 -1, height, width) {
                    if block[i][ri][ci-1] == SimpleCell::EMPTY {
                        next_map[ri][ci-1] = SimpleCell::FULL;
                    }
                }
            }
        }

        if block[i][0][0] == SimpleCell::EMPTY {
            next_map[0][0] = SimpleCell::FULL;
        }
        for ri in 0..height {
            for ci in 0..width {
                current_map[ri][ci] = next_map[ri][ci];
                next_map[ri][ci] = SimpleCell::EMPTY;
            }
        }
        counter = counter +1;
        if current_map[height-1][width-1] == SimpleCell::FULL {
            break;
        }
        i = (i + 1) % block.len();
    }
    return counter.to_string()

}

fn travers2(block: &Vec<Vec<Vec<SimpleCell>>>) -> String {
    
    // Get possible times to start 
    let height = block[0].len();
    let width = block[0][0].len();
    let mut counter = 0;
    let mut i = 0;

    // ----------------------------------------------------------------------------
    // create current time sheet
    let mut current_map = vec![vec![SimpleCell::EMPTY; width]; height];
    let mut next_map = vec![vec![SimpleCell::EMPTY; width]; height];

    
    loop {
        for ri in 0..height {
            for ci in 0..width {
                if current_map[ri][ci] == SimpleCell::EMPTY {
                    continue;
                }
                if block[i][ri][ci] == SimpleCell::EMPTY {
                    next_map[ri][ci] = SimpleCell::FULL;
                }
                if in_bounds(ri as i32 +1, ci as i32, height, width) {
                    if block[i][ri+1][ci] == SimpleCell::EMPTY {
                        next_map[ri+1][ci] = SimpleCell::FULL;
                    }
                }
                if in_bounds(ri as i32 -1, ci as i32, height, width) {
                    if block[i][ri-1][ci] == SimpleCell::EMPTY {
                        next_map[ri-1][ci] = SimpleCell::FULL;
                    }
                }
                if in_bounds(ri as i32, ci as i32 + 1, height, width) {
                    if block[i][ri][ci+1] == SimpleCell::EMPTY {
                        next_map[ri][ci+1] = SimpleCell::FULL;
                    }
                }
                if in_bounds(ri as i32, ci as i32 -1, height, width) {
                    if block[i][ri][ci-1] == SimpleCell::EMPTY {
                        next_map[ri][ci-1] = SimpleCell::FULL;
                    }
                }
            }
        }

        if block[i][0][0] == SimpleCell::EMPTY {
            next_map[0][0] = SimpleCell::FULL;
        }
        for ri in 0..height {
            for ci in 0..width {
                current_map[ri][ci] = next_map[ri][ci];
                next_map[ri][ci] = SimpleCell::EMPTY;
            }
        }
        counter = counter +1;
        i = (i + 1) % block.len();
        if current_map[height-1][width-1] == SimpleCell::FULL {
            break;
        }
    }
    //println!("I broke out: {}", counter);

    // 2 ----------------------------------------------------------------------------
    // create current time sheet
    let mut current_map = vec![vec![SimpleCell::EMPTY; width]; height];
    let mut next_map = vec![vec![SimpleCell::EMPTY; width]; height];

    
    loop {
        for ri in 0..height {
            for ci in 0..width {
                if current_map[ri][ci] == SimpleCell::EMPTY {
                    continue;
                }
                if block[i][ri][ci] == SimpleCell::EMPTY {
                    next_map[ri][ci] = SimpleCell::FULL;
                }
                if in_bounds(ri as i32 +1, ci as i32, height, width) {
                    if block[i][ri+1][ci] == SimpleCell::EMPTY {
                        next_map[ri+1][ci] = SimpleCell::FULL;
                    }
                }
                if in_bounds(ri as i32 -1, ci as i32, height, width) {
                    if block[i][ri-1][ci] == SimpleCell::EMPTY {
                        next_map[ri-1][ci] = SimpleCell::FULL;
                    }
                }
                if in_bounds(ri as i32, ci as i32 + 1, height, width) {
                    if block[i][ri][ci+1] == SimpleCell::EMPTY {
                        next_map[ri][ci+1] = SimpleCell::FULL;
                    }
                }
                if in_bounds(ri as i32, ci as i32 -1, height, width) {
                    if block[i][ri][ci-1] == SimpleCell::EMPTY {
                        next_map[ri][ci-1] = SimpleCell::FULL;
                    }
                }
            }
        }

        if block[i][height-1][width-1] == SimpleCell::EMPTY {
            next_map[height-1][width-1] = SimpleCell::FULL;
        }
        for ri in 0..height {
            for ci in 0..width {
                current_map[ri][ci] = next_map[ri][ci];
                next_map[ri][ci] = SimpleCell::EMPTY;
            }
        }
        counter = counter +1;
        i = (i + 1) % block.len();
        if current_map[0][0] == SimpleCell::FULL {
            break;
        }
    }
    //println!("I broke out 2: {}", counter);

    // 3 ----------------------------------------------------------------------------
    // create current time sheet
    let mut current_map = vec![vec![SimpleCell::EMPTY; width]; height];
    let mut next_map = vec![vec![SimpleCell::EMPTY; width]; height];

    
    loop {
        for ri in 0..height {
            for ci in 0..width {
                if current_map[ri][ci] == SimpleCell::EMPTY {
                    continue;
                }
                if block[i][ri][ci] == SimpleCell::EMPTY {
                    next_map[ri][ci] = SimpleCell::FULL;
                }
                if in_bounds(ri as i32 +1, ci as i32, height, width) {
                    if block[i][ri+1][ci] == SimpleCell::EMPTY {
                        next_map[ri+1][ci] = SimpleCell::FULL;
                    }
                }
                if in_bounds(ri as i32 -1, ci as i32, height, width) {
                    if block[i][ri-1][ci] == SimpleCell::EMPTY {
                        next_map[ri-1][ci] = SimpleCell::FULL;
                    }
                }
                if in_bounds(ri as i32, ci as i32 + 1, height, width) {
                    if block[i][ri][ci+1] == SimpleCell::EMPTY {
                        next_map[ri][ci+1] = SimpleCell::FULL;
                    }
                }
                if in_bounds(ri as i32, ci as i32 -1, height, width) {
                    if block[i][ri][ci-1] == SimpleCell::EMPTY {
                        next_map[ri][ci-1] = SimpleCell::FULL;
                    }
                }
            }
        }

        if block[i][0][0] == SimpleCell::EMPTY {
            next_map[0][0] = SimpleCell::FULL;
        }
        for ri in 0..height {
            for ci in 0..width {
                current_map[ri][ci] = next_map[ri][ci];
                next_map[ri][ci] = SimpleCell::EMPTY;
            }
        }
        counter = counter +1;
        i = (i + 1) % block.len();
        if current_map[height-1][width-1] == SimpleCell::FULL {
            break;
        }
    }
    //println!("I broke out: {}", counter);
    return counter.to_string();

}

pub fn task1(input: &str) -> String {
    let (height, width) = get_input_size(input); 
    let map = parse_input(input, height, width);
    //print_time_slice(&map);
    let block = generate_block(&map);
    //print_time_block(&block);
    travers(&block)
}


pub fn task2(input: &str) -> String {
    let (height, width) = get_input_size(input); 
    let map = parse_input(input, height, width);
    //print_time_slice(&map);
    let block = generate_block(&map);
    //print_time_block(&block);
    travers2(&block)
}



