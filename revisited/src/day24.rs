use ndarray::{Array2, Array3};

type OriginMap = Array2<Option<Origin>>;
type OriginStack = Vec<Origin>;
type BlizzardWorld = Array3<bool>;
type Time = usize;

enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    WAIT
}

struct Path {
    moves: Vec<Move>,
    origin: Origin,
    pos: Pos
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize
}

enum Origin {
    Initial(Time),
    From(Pos)
}



pub fn task1(input: &str) -> String {
    // Read in Blizzard World
    // Initiate Origin Stack

    // from each origin, find all possible reaches (That are none taken), populate the next origin bucket.
    // If target reached, stop.

    // Calculate path
    
    "AAA".to_string()

}


pub fn task2(input: &str) -> String {
    "AAA".to_string()
}

