

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


fn get_direction_iterator(first_direction: Direction)  -> impl Iterator<Item = Direction> {
    let mut m =  vec![Direction::NORTH, Direction::SOUTH, Direction::WEST, Direction::EAST];
    m.into_iter().cycle().skip(match first_direction {
        Direction::NORTH => 0,
        Direction::SOUTH => 1,
        Direction::WEST => 2,
        Direction::EAST => 3
    }).take(4)
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