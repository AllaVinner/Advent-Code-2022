use std::{cmp::max};


#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    WAIT,
    ORE,
    CLAY,
    OBSIDIAN,
    GEODE
}

impl Move {
    fn forbidden(&self, inventory: &Collection, prices: &Prices) -> bool {
        match self {
            Move::WAIT => false,
            Move::ORE => inventory.ore < prices.ore.ore,
            Move::CLAY => inventory.ore < prices.clay.ore,
            Move::OBSIDIAN => inventory.ore < prices.obsidian.ore || inventory.clay < prices.obsidian.clay,
            Move::GEODE => inventory.ore < prices.geode.ore || inventory.obsidian < prices.geode.obsidian,
        }
    }
}

fn create_move_iter_from(m: Option<Move>) -> impl Iterator<Item = Move> + Clone {
    [Move::WAIT, Move::CLAY, Move::ORE, Move::OBSIDIAN, Move::GEODE].into_iter()
        .skip( match m {
            None => 0,
            Some(Move::WAIT) => 1,
            Some(Move::ORE) => 2,
            Some(Move::CLAY) => 3,
            Some(Move::OBSIDIAN) => 4,
            Some(Move::GEODE) => 5,  
        })
}


#[derive(Debug, PartialEq, Clone, Copy)]
struct Collection {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Prices {
    ore: Collection,
    clay: Collection,
    obsidian: Collection,
    geode: Collection
}

fn update(m: Move, inventory: &mut Collection, robots: &mut Collection, prices: &Prices) {
    inventory.ore += robots.ore;
    inventory.clay += robots.clay;
    inventory.obsidian += robots.obsidian;
    inventory.geode += robots.geode;
    match m {
        Move::WAIT => (), 
        Move::ORE => {
            robots.ore += 1;
            inventory.ore -= prices.ore.ore; 
        },
        Move::CLAY => {
            robots.clay += 1;
            inventory.ore -= prices.clay.ore; 
        },
        Move::OBSIDIAN => {
            robots.obsidian += 1;
            inventory.ore -= prices.obsidian.ore; 
            inventory.clay -= prices.obsidian.clay; 
        },
        Move::GEODE => {
            robots.geode += 1;
            inventory.ore -= prices.geode.ore; 
            inventory.obsidian -= prices.geode.obsidian; 
        },
    }
}

fn rollback(m: Move, inventory: &mut Collection, robots: &mut Collection, prices: &Prices) {
    match m {
        Move::WAIT => (), 
        Move::ORE => {
            robots.ore -= 1;
            inventory.ore += prices.ore.ore; 
        },
        Move::CLAY => {
            robots.clay -= 1;
            inventory.ore += prices.clay.ore; 
        },
        Move::OBSIDIAN => {
            robots.obsidian -= 1;
            inventory.ore += prices.obsidian.ore; 
            inventory.clay += prices.obsidian.clay; 
        },
        Move::GEODE => {
            robots.geode -= 1;
            inventory.ore += prices.geode.ore; 
            inventory.obsidian += prices.geode.obsidian; 
        },
    }
    inventory.ore -= robots.ore;
    inventory.clay -= robots.clay;
    inventory.obsidian -= robots.obsidian;
    inventory.geode -= robots.geode;
    
}

fn parse_prices(input: &str) -> Vec<Prices> {
    let mut prices = Vec::new();
    for line in input.lines() {
        let mut trimmed_line = line.replace(".", "").replace(":", "");
        let mut line_iter = trimmed_line.split_whitespace();
        line_iter.next();
        let id: u32 = line_iter.next().unwrap().parse().unwrap();
        line_iter.next();
        line_iter.next();
        line_iter.next();
        line_iter.next();
        let ore_ore: u32 = line_iter.next().unwrap().parse().unwrap();
        line_iter.next();
        line_iter.next();
        line_iter.next();
        line_iter.next();
        line_iter.next();
        let clay_ore: u32 = line_iter.next().unwrap().parse().unwrap();
        line_iter.next();
        line_iter.next();
        line_iter.next();
        line_iter.next();
        line_iter.next();
        let obsidian_ore: u32 = line_iter.next().unwrap().parse().unwrap();
        line_iter.next();
        line_iter.next();
        let obsidian_clay: u32 = line_iter.next().unwrap().parse().unwrap();
        line_iter.next();
        line_iter.next();
        line_iter.next();
        line_iter.next();
        line_iter.next();
        let geode_ore: u32 = line_iter.next().unwrap().parse().unwrap();
        line_iter.next();
        line_iter.next();
        let geode_obsidian: u32 = line_iter.next().unwrap().parse().unwrap();
        prices.push(Prices { ore: Collection { ore: ore_ore, clay: 0, obsidian: 0, geode: 0 }, 
                             clay: Collection { ore: clay_ore, clay: 0, obsidian: 0, geode: 0 },
                             obsidian: Collection { ore: obsidian_ore, clay: obsidian_clay, obsidian: 0, geode: 0 }, 
                            geode: Collection { ore: geode_ore, clay: 0, obsidian: geode_obsidian, geode: 0 } 
                            })
    }
    prices
}

pub fn task1(input: &str) -> String {
    let prices = parse_prices(input);
    for p in prices.iter() {
        let mut inventory = Collection {ore: 0, clay: 0, obsidian: 0, geode: 0};
        let mut robots = Collection {ore: 1, clay: 0, obsidian: 0, geode: 0};
        let mut moves = Vec::<Move>::new();
        let mut update_success = false;
        let mut move_iter = create_move_iter_from(None);
        let mut best_score = 0;
        loop {
            update_success  = false;
            //println!("{:?}",moves.len());
            for m in move_iter.clone() {
                //println!("Move; {:?}", m);
                if m.forbidden(&inventory, p) {
                    continue;
                }
                if moves.len() == 24 {
                    update_success = false;
                    best_score = max(best_score, inventory.geode);
                    break;
                }
                
                update_success = true;
                moves.push(m);
                update(m, &mut inventory, &mut robots, &p);
                move_iter = create_move_iter_from(None);
                break;
            }

            if !update_success {
                let prev_move = match moves.pop() {
                    Some(m) => m,
                    None => break
                };
                rollback(prev_move, &mut inventory, &mut robots, p);
                move_iter = create_move_iter_from(Some(prev_move));
            }

        }
        println!("{:?}", best_score);
    }
    "AAA".to_string()
}



pub fn task2(input: &str) -> String {
    
    "AAA".to_string()
}


