
type time = usize;

enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    WAIT
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize
}

enum Reached {
    NotReached,
    Initial(time),
    ReachedFrom(Pos)
}



pub fn task1(input: &str) -> String {
    "AAA".to_string()
}


pub fn task2(input: &str) -> String {
    "AAA".to_string()
}

