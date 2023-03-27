use std::path::Component;


#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    WAIT,
    ORE,
    CLAY,
    OBSIDIAN,
    GEODE
}

fn create_move_iter() -> impl Iterator<Item = Move>{
    [Move::WAIT, Move::CLAY, Move::ORE, Move::OBSIDIAN, Move::GEODE].into_iter()
}

fn create_move_iter_from(m: Move) -> impl Iterator<Item = Move> {
    create_move_iter().skip( match m {
        Move::WAIT => 1,
        Move::ORE => 2,
        Move::CLAY => 3,
        Move::OBSIDIAN => 4,
        Move::GEODE => 5,  
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

}

fn rollback(m: Move, inventory: &mut Collection, robots: &mut Collection, prices: &Prices) {
    
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
        let mut inventory = Collection {ore: 1, clay: 0, obsidian: 0, geode: 0};
        let mut robots = Collection {ore: 1, clay: 0, obsidian: 0, geode: 0};
        let mut moves = Vec::<Move>::new();
        
        println!("{:?}", p);
    }
    "AAA".to_string()
}



pub fn task2(input: &str) -> String {
    
    "AAA".to_string()
}


