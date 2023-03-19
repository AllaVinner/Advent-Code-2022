use ndarray::{Array2, Array3};
use num::integer::lcm;

type OriginMap = Array2<Option<Origin>>;
type OriginStack = Vec<Origin>;
type BlizzardWorld = Array3<bool>;
type Time = usize;

fn add(pos: Pos, m: &Move) -> Option<Pos> {
    match m {
        Move::WAIT => Some(pos),
        Move::DOWN => Some(Pos { x: pos.x, y: pos.y+1 }),
        Move::RIGHT => Some(Pos { x: pos.x+1, y: pos.y}),
        Move::UP => {
            if pos.y > 0 {
                Some(Pos { x: pos.x, y: pos.y-1})
            } else {
                None
            }
        },
        Move::LEFT => {
            if pos.x > 0 {
                Some(Pos { x: pos.x-1, y: pos.y})
            } else {
                None
            }
        }
    }
}

fn update(path: &mut Path, world: &Array3<bool>) {
    // Current path is valid ...
    // i) Try and extend path
    let mut new_pos;
    let mut blizzard_state;
    loop {
        for m in move_iter() {
            new_pos = match add(path.pos, m) {
                Some(p) => p,
                None => continue
            };
    
            blizzard_state =  world.get((path.len() + 1, new_pos.x, new_pos.y));
            if blizzard_state.is_none() {
                continue;
            }
            if *blizzard_state.unwrap() {
                continue;
            }
            path.moves.push(m.clone());
            return;
        }

        let prev_move = path.moves.pop().unwrap_or_else(|| return);
        for m in iter_from_move(&prev_move) {

        }
    }
    
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    WAIT
}

impl Move {
    fn next(&self) -> Option<Move> {
        match self {
            Move::WAIT => Some(Move::UP),
            Move::UP => Some(Move::RIGHT),
            Move::RIGHT => Some(Move::DOWN),
            Move::DOWN => Some(Move::LEFT),
            Move::LEFT => None
        }
    }
}

fn move_iter() -> std::slice::Iter<'static, Move> {
   [Move::WAIT, Move::UP, Move::RIGHT, Move::DOWN, Move::LEFT].iter()
}

fn iter_from_move(m: &Move) -> std::slice::Iter<'static, Move> {
    let mut moves = move_iter();
    loop {
        let next_move = moves.next();

        if Some(m) == next_move {
            return moves;
        }

        if next_move.is_none() {
            return moves;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Path {
    moves: Vec<Move>,
    origin: Origin,
    pos: Pos
}

impl Path {
    fn len(&self) -> usize {
        match self.origin {
            Origin::From(_) => self.moves.len(),
            Origin::Initial(t) => self.moves.len() + t,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: usize,
    y: usize
}

#[derive(Debug, PartialEq, Clone)]
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
                    Move::DOWN => ((row+time) % initial_world.dim().0, col),
                    Move::RIGHT =>  (row, (col+time) % initial_world.dim().1),
                    Move::UP => ((row+num_time_steps-time) % initial_world.dim().0, col),
                    Move::LEFT =>  (row, (col+num_time_steps-time) % initial_world.dim().1),
                    Move::WAIT => continue
                };
                world[[time, r,c]] = true;
            }
        }
    }
    world
}

fn get_initial_origins(world: &Array3<bool>) -> Vec<Origin> {
    let (period, _, _) = world.dim();
    let mut origins = Vec::new();
    for t in 0..period {
        if world[[t, 0, 0]] {
            continue;
        }
        origins.push(Origin::Initial(t));
    }
    origins
}

fn initialize_path(origin: Origin) -> Path {
    let mut pos = match origin {
        Origin::Initial(t) => Pos {x: 0, y: 0},
        Origin::From(p) => p
    };
    Path {moves: Vec::<Move>::new(), origin, pos}
}

pub fn task1(input: &str) -> String {
    let initial_world = parse_input(input);
    let period = lcm(initial_world.dim().0, initial_world.dim().1);

    let w = create_world(initial_world, period);

    let origins = get_initial_origins(&w);

    for origin in origins.into_iter() {
        println!("Origi {:?}", initialize_path(origin));
    }

    for m in move_iter() {
        println!("Move {:?}", m);
    }

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

