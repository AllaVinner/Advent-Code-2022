use std::cmp::{min, max};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    NORTH,
    SOUTH,
    WEST,
    EAST
}

impl Pos {
    fn step(&self, direction: Direction) -> Pos {
        match direction {
            Direction::NORTH => Pos{x: self.x, y: self.y-1},
            Direction::SOUTH => Pos{x: self.x, y: self.y+1},
            Direction::WEST => Pos{x: self.x-1, y: self.y},
            Direction::EAST => Pos{x: self.x+1, y: self.y}
        }
    }
}

fn get_direction_iterator(first_direction: Direction)  -> impl Iterator<Item = Direction> {
    let m =  vec![Direction::NORTH, Direction::SOUTH, Direction::WEST, Direction::EAST];
    m.into_iter().cycle().skip(match first_direction {
        Direction::NORTH => 0,
        Direction::SOUTH => 1,
        Direction::WEST => 2,
        Direction::EAST => 3
    })
}

fn get_direction_neighbours(pos: &Pos, direction: Direction) -> [Pos; 3] {
    match direction {
        Direction::NORTH => [Pos{x: pos.x-1, y: pos.y-1 }, Pos{x: pos.x, y: pos.y-1}, Pos{x: pos.x+1, y: pos.y-1}],
        Direction::SOUTH => [Pos{x: pos.x-1, y: pos.y+1}, Pos{x: pos.x, y: pos.y+1}, Pos{x: pos.x+1, y: pos.y+1}],
        Direction::WEST => [Pos{x: pos.x-1, y: pos.y-1}, Pos{x: pos.x-1, y: pos.y}, Pos{x: pos.x-1, y: pos.y+1}],
        Direction::EAST => [Pos{x: pos.x+1, y: pos.y-1}, Pos{x: pos.x+1, y: pos.y}, Pos{x: pos.x+1, y: pos.y+1}]
    } 
}


fn parse_input(input: &str) -> Vec<Pos> {
    input.lines()
        .enumerate()
        .map(|(line_i, line)| line.chars().enumerate().filter_map(|(char_i, c)| match c {
            '#' => Some(Pos {x: char_i as i32, y: line_i as i32}),
            '.' => None,
            _ => panic!("Recived unexpected character")
        }).collect::<Vec<Pos>>())
        .flatten()
        .collect::<Vec<Pos>>()
}

fn elf_proposal(pos: &Pos, elves: &Vec<Pos>, first_direction: Direction) -> Pos {
    let mut is_free;
    for direction in get_direction_iterator(first_direction).take(4) {
        is_free = true;
        for neighbour in get_direction_neighbours(pos, direction).into_iter() {
            if elves.contains(&neighbour) {
                is_free = false;
                break;
            }
        }
        if is_free {
            return pos.step(direction);
        }
    }
    return *pos;
}

fn get_extremes(elves: &Vec<Pos>) -> [i32; 4] {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for pos in elves.iter() {
        min_x = min(min_x, pos.x);
        max_x = max(max_x, pos.x);
        min_y = min(min_y, pos.y);
        max_y = max(max_y, pos.y);
    }
    [min_x, max_x, min_y, max_y]
}


pub fn task1(input: &str) -> String {
    let mut elves = parse_input(input);
    let mut proposals: Vec<Pos> = Vec::new();
    for first_direction in get_direction_iterator(Direction::NORTH).take(10) {
        proposals = elves.iter().map(|elf| elf_proposal(elf, &elves, first_direction)).collect();
        for (pos, proposal) in elves.iter_mut().zip(proposals.iter()) {
            if proposals.iter().filter(|p| *p == proposal).count() == 1 {
                *pos = *proposal;
            }
        }
    }

    let [min_x, max_x, min_y, max_y] = get_extremes(&elves);
    let area = (max_x - min_x+1) * (max_y-min_y+1);
    for p in elves.iter() {
        println!("x: {}, y: {}", p.x, p.y);
    }
    
    println!("{:?}", area);
    println!("Num Elves {:?}", elves.len());
    println!("Width {:?}", (max_x - min_x+1));
    println!("Height {:?}", (max_y-min_y+1));
    "ASD".to_string()
}


pub fn task2(input: &str) -> String {

    "ASD".to_string()
}