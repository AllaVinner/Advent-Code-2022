use ndarray::{Array2, Array3};
use num::integer::lcm;

type OriginMap = Array2<Option<Origin>>;
type OriginStack = Vec<Origin>;
type BlizzardWorld = Array3<bool>;
type Time = usize;

#[derive(Debug, Clone, PartialEq)]
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

fn parse_input(input: &str) -> Array2<Move>{
    let width = input.lines().nth(0).unwrap().chars().count() - 2;
    let height = input.lines().count() - 2;

    let mut blizzards= Array2::from_elem([height, width], Move::WAIT);
    for (row, line) in input.lines().skip(1).enumerate() {
        if row == height {
            break;
        }
        for (col, c) in line.chars().skip(1).enumerate(){
            blizzards[[row, col]] = match c {
                '<' => Move::LEFT,
                '^' => Move::UP,
                '>' => Move::RIGHT,
                'v' => Move::DOWN,
                '.' => Move::WAIT,
                _ => continue, 
            };
        }
    }
    blizzards
}


fn create_world(initial_world: Array2<Move>, num_time_steps: usize) -> Array3<bool> {
    let mut world = Array3::from_elem((num_time_steps, initial_world.dim().0, initial_world.dim().1), false);
    for row in 0..initial_world.dim().0 {
        for col in 0..initial_world.dim().1 {
            let direction = &initial_world[[row, col]];
            if direction == &Move::WAIT {
                continue;
            }
            for time in 0..num_time_steps {
                let (r, c) = match direction {
                    Move::DOWN => ((row+time) % num_time_steps, col),
                    Move::RIGHT =>  (row, (col+time) % num_time_steps),
                    Move::UP => ((row+num_time_steps-time) % num_time_steps, col),
                    Move::LEFT =>  (row, (col+num_time_steps-time) % num_time_steps),
                    Move::WAIT => continue
                };
                world[[time, r,c]] = true;
            }
        }
    }
    world
}


pub fn task1(input: &str) -> String {
    let initial_world = parse_input(input);
    let period = lcm(initial_world.dim().0, initial_world.dim().1);

    println!("{:?}", initial_world.dim());
    println!("{:?}", lcm(initial_world.dim().0, initial_world.dim().1));
    
    let w = create_world(initial_world, period);
    println!( "{:?}", w);
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

    /*
    for origin in origins {
        path = Path::new(origin);
        moves.reset();

        loop {
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
            if is_blizzard.is_none()  || is_blizzard.unwrap() {
                path.pop();
                continue;
            }
            if is_goal(path.pos()) {
                path.pop()
                // Some more things
                continue;
            }

            if (path.length() == period) {
                reach_map.add(path.pos())
                path.pop();
                continue;
            }

            move.reset();

        }

    }
    
     */



    "AAA".to_string()
}


pub fn task2(input: &str) -> String {
    "AAA".to_string()
}

