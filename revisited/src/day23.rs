

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
        Direction::EAST => [Pos{x: pos.x+1, y: pos.y-1}, Pos{x: pos.x+1, y: pos.y}, Pos{x: pos.x+1, y: pos.y-1}]
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



pub fn task1(input: &str) -> String {
    let mut elves = parse_input(input);
    let mut first_direction = get_direction_iterator(Direction::NORTH);
    let mut proposals: Vec<Pos> = elves.iter().map(|elf| elf_proposal(elf, &elves, Direction::NORTH)).collect();


    println!("{:?}", elves);
    println!("{:?}", proposals);

    "ASD".to_string()
}


pub fn task2(input: &str) -> String {

    "ASD".to_string()
}