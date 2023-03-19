use ndarray::{Array2, Array3};
use num::integer::lcm;
use std::cmp::min;

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

fn sub(pos: Pos, m: &Move) -> Option<Pos> {
    match m {
        Move::WAIT => Some(pos),
        Move::UP => Some(Pos { x: pos.x, y: pos.y+1 }),
        Move::LEFT => Some(Pos { x: pos.x+1, y: pos.y}),
        Move::DOWN => {
            if pos.y > 0 {
                Some(Pos { x: pos.x, y: pos.y-1})
            } else {
                None
            }
        },
        Move::RIGHT => {
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
    let mut iter_move = if path.moves.len() == world.dim().0 {
        iter_from_move(&path.moves.pop().unwrap())
    } else {
        move_iter()
    };
    loop {
        println!("Starting update with: {:?}", &path);
        for m in iter_move {
            new_pos = match add(path.pos, m) {
                Some(p) => p,
                None => continue
            };
            println!("New pos was {:?}", new_pos);
    
            blizzard_state = world.get((path.len() + 1, new_pos.x, new_pos.y));
            if blizzard_state.is_none() {
                continue;
            }
            if *blizzard_state.unwrap() {
                continue;
            }
            println!("Move was choosen, {:?}, world index {:?}", m, (path.len() + 1, new_pos.x, new_pos.y));

            path.moves.push(m.clone());
            path.pos = new_pos;
            return;
        }
        println!("No move was good");
        let prev_move = match path.moves.pop() {
            None => return,
            Some(m) => m,
        };
        path.pos = match sub(path.pos, &prev_move) {
            Some(p) => p,
            None => panic!("Subtracted move which cause the move to be invalid.")
        };

        iter_move = iter_from_move(&prev_move);
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Origin {
    Initial(Time),
    From(Pos)
}

fn parse_input(input: &str) -> Array2<Move>{
    let width = input.lines().nth(0).unwrap().chars().count() - 2;
    let height = input.lines().count() - 2;

    let mut blizzards= Array2::from_elem([width, height], Move::WAIT);
    for (row, line) in input.lines().skip(1).enumerate() {
        if row == height {
            break;
        }
        for (col, c) in line.chars().skip(1).enumerate(){
            blizzards[[col, row]] = match c {
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
    for col in 0..initial_world.dim().0 {
        for row in 0..initial_world.dim().1 {
            let direction = &initial_world[[col, row]];
            if direction == &Move::WAIT {
                continue;
            }
            for time in 0..num_time_steps {
                let (r, c) = match direction {
                    Move::DOWN => ((row+time) % initial_world.dim().1, col),
                    Move::RIGHT =>  (row, (col+time) % initial_world.dim().0),
                    Move::UP => ((row+num_time_steps-time) % initial_world.dim().1, col),
                    Move::LEFT =>  (row, (col+num_time_steps-time) % initial_world.dim().0),
                    Move::WAIT => continue
                };
                world[[time, c, r]] = true;
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

fn initialize_path(origin: &Origin) -> Path {
    let mut pos = match *origin {
        Origin::Initial(t) => Pos {x: 0, y: 0},
        Origin::From(p) => p
    };
    Path {moves: Vec::<Move>::new(), origin: origin.clone(), pos }
}

pub fn task1(input: &str) -> String {
    let initial_world = parse_input(input);
    let period = lcm(initial_world.dim().0, initial_world.dim().1);
    let goal = Pos{x: initial_world.dim().0 -1, y: initial_world.dim().1-1};

    let mut reached_map = Array2::<Option<Origin>>::from_elem([initial_world.dim().0, initial_world.dim().1], None);
    let world = create_world(initial_world, period);

    let mut origins = get_initial_origins(&world);
    let mut next_origins = Vec::<Origin>::new();
    let mut goal_reached = false;

    reached_map[[0,0]] = Some(origins[0]);
    
    let mut best_value = usize::MAX;
    let mut num_cycles = 0;
    loop {
        next_origins = Vec::<Origin>::new();
        for origin in origins {
            println!("Starting from origin {:?}", origin);
            let mut path = initialize_path(&origin);
            loop {
                update(&mut path, &world);
                println!("Currently checking path: {:?}", &path);
                if path.pos == goal {
                    best_value = min(path.len() + period*num_cycles, best_value);
                    goal_reached = true;
                    continue;
                }
                if path.moves.len() == period {
                    if reached_map[[path.pos.x, path.pos.y]].is_none() {
                        reached_map[[path.pos.x, path.pos.y]] = Some(path.origin);
                        next_origins.push(Origin::From(path.pos));
                    }
                }
                if path.moves.len() == 0 {
                    break;
                }      
            }
        }
        if goal_reached {
            break;
        }
        println!("Period done, current reached mapped {:?}", &reached_map);
        num_cycles = num_cycles + 1;
        origins = next_origins;
        break;
    }
    println!("Best path found... {:?}", best_value);

    "AAA".to_string()
}


pub fn task2(input: &str) -> String {
    "AAA".to_string()
}

