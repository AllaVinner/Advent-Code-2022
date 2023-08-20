use std::collections::HashMap;
use std::cmp;
use std::ops::{Add, Sub};


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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Collection {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32
}

impl Add for Collection {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Collection {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode
        }
    }
}


impl Sub for Collection {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Collection {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode
        }
    }
}

impl Collection {
    fn remove_ores(self, num: u32) -> Self {
        Collection {
            ore: self.ore - num,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode
        }
    }

    fn remove_clays(self, num: u32) -> Self {
        Collection {
            ore: self.ore,
            clay: self.clay - num,
            obsidian: self.obsidian,
            geode: self.geode
        }
    }
    fn remove_obsidians(self, num: u32) -> Self {
        Collection {
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian - num,
            geode: self.geode
        }
    }
    // ---------
    fn add_ores(self, num: u32) -> Self {
        Collection {
            ore: self.ore + num,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode
        }
    }

    fn add_clays(self, num: u32) -> Self {
        Collection {
            ore: self.ore,
            clay: self.clay + num,
            obsidian: self.obsidian,
            geode: self.geode
        }
    }
    fn add_obsidians(self, num: u32) -> Self {
        Collection {
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian + num,
            geode: self.geode
        }
    }

    fn add_geode(self, num: u32) -> Self {
        Collection {
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode + num
        }
    }
    
    
}

type ProductInventory = Collection;
type MachineInventory = Collection;

#[derive(Debug, PartialEq, Clone, Copy)]
struct MachinePrices {
    ore: Collection,
    clay: Collection,
    obsidian: Collection,
    geode: Collection
}

type Time = u32;
type Score = u32;

fn max_geom(
    machine_cost: &MachinePrices,
    remaining_time: Time, 
    mut gems: ProductInventory,
    mut machines: MachineInventory,
    set: &mut HashMap< (Time, ProductInventory, MachineInventory), Score>
) -> Score {
    if remaining_time == 0 {
        //println!("At time 0");
        return gems.geode;
    }

    let max_ore_cost = *vec![machine_cost.ore.ore, machine_cost.clay.ore, machine_cost.obsidian.ore, machine_cost.geode.ore].iter().max().unwrap();
    
    machines.ore = cmp::min(machines.ore, max_ore_cost);
    machines.clay = cmp::min(machines.clay, machine_cost.obsidian.clay);
    machines.obsidian = cmp::min(machines.obsidian, machine_cost.geode.obsidian);
    
    gems.ore = cmp::min(gems.ore, remaining_time*max_ore_cost - machines.ore*(remaining_time -1));
    gems.clay = cmp::min(gems.clay, remaining_time*machine_cost.obsidian.clay - machines.clay*(remaining_time -1));
    gems.obsidian = cmp::min(gems.obsidian, remaining_time*machine_cost.geode.obsidian - machines.obsidian*(remaining_time -1));

    let hashable = (remaining_time, gems, machines);
    match set.get(&hashable) {
        Some(best) => return *best,
        None => ()
    }

    let mut best;
    let mut candidate;
    //println!("wating");
    best = max_geom(
        machine_cost, remaining_time -1, 
        gems + machines,
        machines,
        set
    );

    if gems.ore >= machine_cost.geode.ore && gems.obsidian >= machine_cost.geode.obsidian {
        //println!("buying geo");
        candidate = max_geom(
            machine_cost, remaining_time -1, 
            gems + machines - machine_cost.geode,
            machines.add_geode(1),
            set
        );
        if candidate > best {
            best = candidate;
        }   
    }

    if machines.obsidian < machine_cost.geode.obsidian{
        if gems.ore >= machine_cost.obsidian.ore && gems.clay >= machine_cost.obsidian.clay {
            //println!("buying geo");
            candidate =  max_geom(
                machine_cost, remaining_time -1, 
                gems + machines - machine_cost.obsidian,
                machines.add_obsidians(1),
                set
            );
            if candidate > best {
                best = candidate;
            }   
        }
      
    }
    
    if machines.clay < machine_cost.obsidian.clay {
        if gems.ore >= machine_cost.clay.ore {
            //println!("buying geo");
            candidate =  max_geom(
                machine_cost, remaining_time -1, 
                gems + machines - machine_cost.clay,
                machines.add_clays(1),
                set
            );
            if candidate > best {
                best = candidate;
            }   
        }
      
    }
    
    if machines.ore < max_ore_cost {
        if gems.ore >= machine_cost.ore.ore {
            candidate =  max_geom(
                machine_cost, remaining_time -1, 
                gems + machines - machine_cost.ore,
                machines.add_ores(1),
                set
            );
            if candidate > best {
                best = candidate;
            }   

        }
    }

    set.insert(hashable, best);
    return best;
}



pub fn task1(input: &str) -> String {
    let blueprints = parse_prices(input);
    let mut val = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        let mut set = HashMap::new();
        let best = max_geom(
            blueprint,
            24,
            ProductInventory{ore: 0, clay: 0, obsidian: 0, geode: 0},
            MachineInventory{ore: 1, clay: 0, obsidian: 0, geode: 0},
            &mut set
        );
        val += (i+1) as u32*best;
    }

    val.to_string()
}


pub fn task2(input: &str) -> String {
    let blueprints = parse_prices(input);
    let mut val = 1;
    for blueprint in blueprints.iter().take(3) {
        let mut set = HashMap::new();
        let best = max_geom(
            blueprint,
            32,
            ProductInventory{ore: 0, clay: 0, obsidian: 0, geode: 0},
            MachineInventory{ore: 1, clay: 0, obsidian: 0, geode: 0},
            &mut set
        );
        println!("{}", best);
        val *= best;
    }

    val.to_string()
}



