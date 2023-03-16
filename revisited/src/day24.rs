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

fn parse_input(input: &str) {
    let width = input.lines().nth(0).unwrap().chars().count() - 2;
    let height = input.lines().count() - 2;
}



pub fn task1(input: &str) -> String {
    

    println!("{:?}", width);
    println!("{:?}", height);
    
    // For each origin
    // Init path with origin
    // while true
    // get next move,
    // 
    // Options
    // i) If no-more next move, Remove top, Set iterator to top+1
    // ii) It is an invalid path -> discard move, continue
    // iii) Is valid and destination reached ....
    // iv) Period time is reached ...
    // i) It is a valid path -> add move, reset move iterator, Continue


    for origin in origins {
        path = Path::new(origin);
        moves.reset();

        while (true) {
            move = moves.next();
            if move.is_none() {
                last_move = path.pop();
                if path.is_exhausted() {
                    break;
                }
                moves.set(last_move);
                continue;
            }
            path.add(move.unwrap());
            is_blizzard = World.get(path.pos());
            if is_blizzard.is_none() {
                path.pop();
                continue;
            }

            

        }

    }
    


    "AAA".to_string()

}


pub fn task2(input: &str) -> String {
    "AAA".to_string()
}

