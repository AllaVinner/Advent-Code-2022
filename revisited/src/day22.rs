use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Line {
    start: u32,
    len: u32
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Agent {
    pos: Pos,
    direction: Direction
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Rotation {
    LEFT,
    RIGHT
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    NORHT,
    SOUTH,
    WEST,
    EAST
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Instruction {
    TURN(Rotation),
    GO(u32)
}

impl Line {
    fn inside(&self, i: u32) -> bool {
        self.start <= i && i < self.start+self.len
    }
}

fn normalize(i: i32, line: Line) -> i32 {
    let a = ((i+((line.start*(line.len-1)+line.len) as i32)) % (line.len as i32)) + (line.start as i32);
    a
}

struct Map {
    obstacles: HashSet<Pos>,
    rows: Vec<Line>,
    cols: Vec<Line>,
}

impl Pos {
    fn step(& self, direction: Direction, map: &Map) -> Pos {
        match direction {
            Direction::NORHT => Pos{ x: self.x, y: normalize(self.y-1, map.cols[self.x as usize]) as i32},
            Direction::WEST => Pos{ x: normalize(self.x-1, map.rows[self.y as usize]) as i32, y: self.y},
            Direction::SOUTH => Pos{ x: self.x, y: normalize(self.y+1, map.cols[self.x as usize]) as i32},
            Direction::EAST => Pos{ x: normalize(self.x+1, map.rows[self.y as usize]) as i32, y: self.y}
        }
    }
}


impl Agent {

    fn turn(&mut self, rot: Rotation) {
    let d = match self.direction {
            Direction::NORHT => 0,
            Direction::EAST => 1,
            Direction::SOUTH => 2,
            Direction::WEST => 3,
        };
    let a = match rot {
        Rotation::LEFT => -1,
        Rotation::RIGHT => 1
    };

    self.direction=  match (d+a+4) % 4 {
        0 => Direction::NORHT,
        1 => Direction::EAST,
        2 => Direction::SOUTH,
        3 => Direction::WEST,
        _ => panic!("GOt number out of 0 to 4 range")
    }       
    }

    fn step(& mut self, steps: u32, map: &Map) {
        for _ in 0..steps {
            let proposal = self.pos.step(self.direction, map);
            if map.obstacles.contains(&proposal) {
                break;
            }
            self.pos = proposal;
        }
    }

    fn follow(& mut self, instruction: Instruction, map: &Map) {
        match instruction {
            Instruction::TURN(rot) => self.turn(rot),
            Instruction::GO(steps) => self.step(steps, map),
        };
    }
}


fn parse_objects(input: &str) -> HashSet<Pos> {
    input.lines()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().filter_map(|(x, c)| match c {
            '#' => Some(Pos {x: x as i32, y: y as i32}),
            _ => None
        }).collect::<HashSet<Pos>>())
        .flatten()
        .collect::<HashSet<Pos>>()
}

fn parse_metadata(input: &str) -> (Vec<Line>, Vec<Line>) {
    let mut row_defs: Vec<Line> = input.lines()
        .map(| line| {
            let start = line.chars().position(|c| c != ' ').unwrap();
            Line {start: start as u32, len: (line.chars().count() - start) as u32 }
        })
        .collect();

    let num_col = input.lines().map(|line| line.chars().count()).max().unwrap();
    let mut col_def: Vec<Line> = Vec::with_capacity(num_col);

    let col_defs: Vec<Line> = (0..num_col).into_iter()
        .map(|col_i| {
            let start = row_defs.iter().position(|rd| rd.inside(col_i as u32)).unwrap();
            let end = row_defs.len() - row_defs.iter().rev().position(|rd| rd.inside(col_i as u32)).unwrap();
            Line {start: start as u32, len: (end-start) as u32}
        })
        .collect();
    (row_defs, col_defs)
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut current_num = 0;
    let mut instructions = Vec::<Instruction>::new();
    for c in input.chars() {
        match c {
            'R' => {
                if current_num != 0 {
                    instructions.push(Instruction::GO(current_num));
                }
                current_num = 0;
                instructions.push(Instruction::TURN(Rotation::RIGHT))
            },
            'L' => {
                if current_num != 0 {
                    instructions.push(Instruction::GO(current_num));
                }
                current_num = 0;
                instructions.push(Instruction::TURN(Rotation::LEFT))
            },
            n => {
                current_num = 10*current_num + n.to_digit(10).unwrap();
            }
        }
    }
    if current_num > 0 {
        instructions.push(Instruction::GO(current_num));
    }
    instructions
}

fn score(agent: &Agent) -> i32 {
    1000*(agent.pos.y+1) + 4*(agent.pos.x+1) + match agent.direction {
        Direction::EAST => 0,
        Direction::SOUTH => 1,
        Direction::WEST => 2,
        Direction::NORHT => 3,
    }
}


pub fn task1(input: &str) -> String {
    let (map_input, instruction_input) = input.split_once("\n\n").unwrap();
    let obstacles = parse_objects(map_input);
    let (rows, cols) = parse_metadata(map_input);
    let mut agent = Agent {pos : Pos{x: rows[0].start as i32, y: 0}, direction: Direction::EAST};
    
    let map = Map{obstacles, rows, cols};

    let instructions = parse_instructions(instruction_input);

   
    for instruction in instructions.iter() {
        agent.follow(*instruction, &map);
    }
    

    score(&agent).to_string()
}


pub fn task2(input: &str) -> String {

    "AAA".to_string()
}






