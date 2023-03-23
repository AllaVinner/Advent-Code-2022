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


fn parse_input(input: &str) -> HashSet<Pos> {
    input.lines()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().filter_map(|(x, c)| match c {
            '#' => Some(Pos {x: x as i32, y: y as i32}),
            _ => None
        }).collect::<HashSet<Pos>>())
        .flatten()
        .collect::<HashSet<Pos>>()
}

fn parse_metadata(input: &str) {
    let mut row_def: Vec<Line> = input.lines()
        .enumerate()
        .map(|(y, line)| {
            let start = line.chars().position(|c| c != ' ').unwrap();
            Line {start: start as u32, len: (line.chars().count() - start) as u32 }
        })
        .collect();
        
}


pub fn task1(input: &str) -> String {
    "AAA".to_string()
}


pub fn task2(input: &str) -> String {

    "AAA".to_string()
}






