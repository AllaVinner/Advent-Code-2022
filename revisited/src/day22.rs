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

fn parse_metadata(input: &str) -> (Vec<Line>, Vec<Line>) {
    let mut row_defs: Vec<Line> = input.lines()
        .map(| line| {
            let start = line.chars().position(|c| c != ' ').unwrap();
            Line {start: start as u32, len: (line.chars().count() - start) as u32 }
        })
        .collect();
    let num_col = input.lines().next().unwrap().chars().count();
    let mut col_def: Vec<Line> = Vec::with_capacity(num_col);
    let col_defs: Vec<Line> = (0..num_col).into_iter()
        .map(|col_i| {
            let start = row_defs.iter().position(|rd| rd.start <= col_i as u32).unwrap();
            let end = row_defs.iter().position(|rd| col_i as u32 <= rd.start + rd.len).unwrap();
            Line {start: start as u32, len: (end-start) as u32}
        })
        .collect();
    (row_defs, col_defs)
}


pub fn task1(input: &str) -> String {
    let md = parse_metadata(input);
    println!("{:?}", md);
    "AAA".to_string()
}


pub fn task2(input: &str) -> String {

    "AAA".to_string()
}






