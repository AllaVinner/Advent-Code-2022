use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Line {
    start: u32,
    len: u32
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Rotation {
    LEFT,
    RIGHT
}


#[derive(Debug, PartialEq, Eq, Hash)]
enum Instruction {
    RIGHT,
    LEFT,
    GO(u32)
}

impl Line {
    fn inside(&self, i: u32) -> bool {
        self.start <= i && i < self.start+self.len
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
                instructions.push(Instruction::RIGHT)
            },
            'L' => {
                if current_num != 0 {
                    instructions.push(Instruction::GO(current_num));
                }
                current_num = 0;
                instructions.push(Instruction::LEFT)
            },
            n => {
                current_num = 10*current_num + n.to_digit(10).unwrap();
            }
        }
    }
    instructions
}


pub fn task1(input: &str) -> String {
    let (map_input, instruction_input) = input.split_once("\n\n").unwrap();
    let objects = parse_objects(map_input);
    let (row, col) = parse_metadata(map_input);
    let instructions = parse_instructions(instruction_input);
    println!("{:?}", col);
    println!("{:?}", objects);
    "AAA".to_string()
}


pub fn task2(input: &str) -> String {

    "AAA".to_string()
}






