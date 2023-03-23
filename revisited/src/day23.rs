

#[derive(Debug, PartialEq)]
struct Pos {
    x: i32,
    y: i32
}

enum Direction {
    NORTH,
    SOUTH,
    WEST,
    EAST
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


pub fn task1(input: &str) -> String {
    let mut elves = parse_input(input);
    println!("{:?}", elves);
    "ASD".to_string()
}


pub fn task2(input: &str) -> String {

    "ASD".to_string()
}