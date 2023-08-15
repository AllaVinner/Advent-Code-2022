use std::ops::{Sub, Add};


#[derive(Debug, PartialEq, Clone, Copy)]
enum Stone {
    Ore,
    Clay,
    Obsidian,
    Geode
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Decision {
    WAIT,
    BUY(Stone)
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Collection {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct MachinePrices {
    ore: Collection,
    clay: Collection,
    obsidian: Collection,
    geode: Collection
}

#[derive(Debug, Clone)]
struct Cursor {
    inventory: Collection,
    machines: Collection,
    decisions: Vec<Decision>,
    prices: MachinePrices,
    max_score: u32,
    max_iter: usize
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ActionResult {
    SUCCESS,
    FAILURE
}

impl MachinePrices {

    fn get_price_of(&self, stone: Stone) -> &Collection{
        match stone {
            Stone::Ore => &self.ore,
            Stone::Clay => &self.clay,
            Stone::Obsidian => &self.obsidian,
            Stone::Geode => &self.geode
        }
    }
}


impl Collection {

    fn is_enough_for(&self, other: &Self) -> bool {
        if self.ore < other.ore {
            return false;
        }
        if self.clay < other.clay {
            return false;
        }
        if self.obsidian < other.obsidian {
            return false;
        }
        if self.geode < other.geode {
            return false;
        }
        return true;
    }
}

impl Sub<&Collection> for Collection {
    type Output = Self;

    fn sub(self, other: &Self) -> Self::Output {
        Collection{
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode
        }
    }
}

impl Add<&Collection> for Collection {
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        Collection{
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode
        }
    }
}


impl Decision {

    fn next(&self) -> Option<Decision> {
        match self {
            Decision::WAIT => Some(Decision::BUY(Stone::Ore)),
            Decision::BUY(s) =>  {
                let next_stone = match s { 
                    Stone::Ore => Some(Stone::Clay),
                    Stone::Clay => Some(Stone::Obsidian),
                    Stone::Obsidian => Some(Stone::Geode),
                    Stone::Geode => None
                };
                match next_stone {
                    None => None,
                    Some(s) => Some(Decision::BUY(s))
                }
            }
        }
    }

}

impl Cursor {

    fn accumulate_current_state(&mut self) {
        if self.inventory.geode > self.max_score {
            self.max_score = self.inventory.geode;
        }
    }

    fn init_decision(&self) -> Decision {
        Decision::WAIT
    }

    fn update(&mut self, decision: Decision) -> ActionResult {
        if self.decisions.len() == self.max_iter {
            return ActionResult::FAILURE;
        }


        //pruning
        if decision != Decision::BUY(Stone::Geode) {
            if self.inventory.is_enough_for(self.prices.get_price_of(Stone::Geode)) {
                return ActionResult::FAILURE;
            }
        }

        if self.machines.ore == 0 && decision != Decision::BUY(Stone::Ore) && self.inventory.is_enough_for(self.prices.get_price_of(Stone::Ore)) {
            return ActionResult::FAILURE;
        }
        if self.machines.clay == 0 && decision != Decision::BUY(Stone::Clay) && self.inventory.is_enough_for(self.prices.get_price_of(Stone::Clay)) {
            return ActionResult::FAILURE;
        }
        if self.machines.obsidian == 0 && decision != Decision::BUY(Stone::Obsidian) && self.inventory.is_enough_for(self.prices.get_price_of(Stone::Obsidian)) {
            return ActionResult::FAILURE;
        }
        if self.machines.geode == 0 && decision != Decision::BUY(Stone::Geode) && self.inventory.is_enough_for(self.prices.get_price_of(Stone::Geode)) {
            return ActionResult::FAILURE;
        }

        if 


        // end pruning
        match decision {
            Decision::WAIT => (),
            Decision::BUY(s) => {
                if ! self.inventory.is_enough_for(self.prices.get_price_of(s)) {
                    return ActionResult::FAILURE;
                }
                self.inventory = self.inventory - self.prices.get_price_of(s);
            }
        }

        self.inventory = self.inventory + &self.machines;

        match decision {
            Decision::WAIT => (),
            Decision::BUY(s) => {
                match s {
                    Stone::Ore => self.machines.ore +=1,
                    Stone::Clay => self.machines.clay +=1,
                    Stone::Obsidian => self.machines.obsidian +=1,
                    Stone::Geode => self.machines.geode +=1,
                }
            }
        }
        
    
        self.decisions.push(decision);

        ActionResult::SUCCESS
    }

    fn next(&self, decision: Decision) -> Option<Decision> {
        decision.next()
    }

    fn rewind(&mut self) -> Option<Decision> {
        let decision = match self.decisions.pop() {
            None => return None,
            Some(d) => d
        };

        match decision {
            Decision::WAIT => (),
            Decision::BUY(s) => {
                match s {
                    Stone::Ore => self.machines.ore -=1,
                    Stone::Clay => self.machines.clay -=1,
                    Stone::Obsidian => self.machines.obsidian -=1,
                    Stone::Geode => self.machines.geode -=1,
                }
            }
        }

        self.inventory = self.inventory - &self.machines;

        match decision {
            Decision::WAIT => (),
            Decision::BUY(s) => {
                self.inventory = self.inventory + self.prices.get_price_of(s);
            }
        }

        return Some(decision);
        
    }


    fn accumulated_state(& self) -> u32 {
        self.max_score
    }

}



fn parse_prices(input: &str) -> Vec<MachinePrices> {
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
        prices.push(MachinePrices { ore: Collection { ore: ore_ore, clay: 0, obsidian: 0, geode: 0 }, 
                             clay: Collection { ore: clay_ore, clay: 0, obsidian: 0, geode: 0 },
                             obsidian: Collection { ore: obsidian_ore, clay: obsidian_clay, obsidian: 0, geode: 0 }, 
                            geode: Collection { ore: geode_ore, clay: 0, obsidian: geode_obsidian, geode: 0 } 
                            })
    }
    prices
}


fn initialize_cursor(machine_prices: MachinePrices) -> Cursor {
    Cursor {inventory: Collection { ore: 0, clay: 0, obsidian: 0, geode: 0, }, 
            machines: Collection { ore: 1, clay: 0, obsidian: 0, geode: 0 },
            decisions: Vec::new(), prices: machine_prices,
            max_score: 0,
            max_iter: 24}
}

fn tree_accumulation(mut cursor: Cursor) -> u32 {
    let mut i = 0;
    loop {
        if i % 10000000 == 0 {
            println!("At {}, with max {}, with decisions: {:?} ", i, cursor.accumulated_state(), cursor.decisions);
        }
        i +=1;
        // Do stuff
        cursor.accumulate_current_state();
        let mut decision = cursor.init_decision();
        
        loop {
            //println!("{:?}", cursor.decisions);
            //println!("Inventory {:?}", cursor.inventory);
            match cursor.update(decision) {
                ActionResult::SUCCESS => {
                    //println!("Updated succeeded");
                    break;
                },
                ActionResult::FAILURE => {
                    //println!("Updated Failed");
                    loop {
                        //println!(" ");
                        //println!("Current Decision {:?} ", decision);
                        //println!("Current inventory {:?} ", cursor.inventory);
                        
                        decision = match cursor.next(decision) {
                            Some(d) => d,
                            None => {
                                match cursor.rewind() {
                                    None => return cursor.accumulated_state(),
                                    Some(d) => {
                                        decision = d;
                                        continue;
                                    }
                                }
                            }
                        };
                        break;
                    }
                }
            }
        }
    }
}

pub fn task1(input: &str) -> String {
    let prices_vec = parse_prices(input);
    for prices in prices_vec.into_iter() {
        let cursor = initialize_cursor(prices);
        println!("{:?}", tree_accumulation(cursor));
    }
    "too".to_string()
}



pub fn task2(input: &str) -> String {
    "aa".to_string()
}
