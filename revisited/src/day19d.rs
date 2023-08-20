use std::collections::HashMap;
use std::{cmp, default};
use std::ops::{Add, Sub, Index, IndexMut};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;


#[derive(Debug, EnumIter, Copy, Clone, PartialEq, Eq, Hash)]
enum Gem {
    Ore,
    Clay,
    Obsidian,
    Geode
}

// Boiler Plate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GemCollection<T> {
    ore: T,
    clay: T,
    obsidian: T,
    geode: T
}

impl<T> Index<Gem> for GemCollection<T> {
    type Output = T;

    fn index(&self, item: Gem) -> &Self::Output {
        match item {
            Gem::Ore => &self.ore,
            Gem::Clay => &self.clay,
            Gem::Obsidian => &self.obsidian,
            Gem::Geode => &self.geode
            
        }
    }
}

impl<T> IndexMut<Gem> for GemCollection<T> {

    fn index_mut(&mut self, item: Gem) -> &mut Self::Output {
        match item {
            Gem::Ore => &mut self.ore,
            Gem::Clay => &mut self.clay,
            Gem::Obsidian => &mut self.obsidian,
            Gem::Geode => &mut self.geode
            
        }
    }
}

impl Default for GemCollection<Option<u32>> {
    fn default() -> Self {
        GemCollection{
            ore: None,
            clay: None,
            obsidian: None,
            geode: None
        }
    }
}

// End Boiler Plate

impl<T: Add<Output = T> + Clone + Copy> Add for GemCollection<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_coll = self.clone();
        for g in Gem::iter() {
            new_coll[g] = new_coll[g] + rhs[g];
        }
        new_coll
    }
}

impl<T: Sub<Output = T> + Clone + Copy> Sub for GemCollection<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut new_coll = self.clone();
        for g in Gem::iter() {
            new_coll[g] = new_coll[g] - rhs[g];
        }
        new_coll
    }
}

type GemInventory = GemCollection<u32>;
type MachineInventory = GemCollection<u32>;
type MachinePrices = GemCollection<GemInventory>;


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
        prices.push(MachinePrices { ore: MachineInventory { ore: ore_ore, clay: 0, obsidian: 0, geode: 0 }, 
                             clay: MachineInventory { ore: clay_ore, clay: 0, obsidian: 0, geode: 0 },
                             obsidian: MachineInventory { ore: obsidian_ore, clay: obsidian_clay, obsidian: 0, geode: 0 }, 
                            geode: MachineInventory { ore: geode_ore, clay: 0, obsidian: geode_obsidian, geode: 0 } 
                            })
    }
    prices
}


impl GemInventory {
    fn exclusive_ge(&self, other: &Self) -> bool {
        for g in Gem::iter(){
            if self[g] < other[g] {
                return false;
            }
        }
        return true;
    }

    fn create_and_add(&self, gem: Gem, num: u32) -> Self {
        let mut new = self.clone();
        new[gem] = new[gem] + num;
        new
    }
}

type Time = u32;
type Count = u32;


fn max_geom(
    machine_cost: & GemCollection<GemCollection<Count>>,
    max_costs: & GemCollection<Option<Count>>,
    remaining_time: Time, 
    mut gems: GemInventory,
    mut machines: MachineInventory,
    set: &mut HashMap< (Time, GemInventory, MachineInventory), Count>
) -> Count {
    if remaining_time == 0 {
        //println!("At time 0");
        return gems.geode;
    }

    for g in Gem::iter() {
        match max_costs[g] {
            None => continue,
            Some(max_cost) => machines[g] = cmp::min(machines[g], max_cost)
        }
    }
    
    for g in Gem::iter() {
        match max_costs[g] {
            None => continue,
            Some(max_cost) => gems[g] = cmp::min(gems[g], remaining_time*max_cost - machines[g]*(remaining_time -1))
        }
    }
    

    let hashable = (remaining_time, gems, machines);
    match set.get(&hashable) {
        Some(best) => return *best,
        None => ()
    }

    let mut best;
    let mut candidate;
    //println!("wating");
    best = max_geom(
        machine_cost, max_costs, remaining_time -1, 
        gems + machines,
        machines,
        set
    );

    for g in Gem::iter() {
        if max_costs[g].is_none() || machines[g] < max_costs[g].unwrap() {
            if gems.exclusive_ge(&machine_cost[g]) {
                candidate =  max_geom(
                    machine_cost, max_costs, remaining_time -1, 
                    gems + machines - machine_cost[g],
                    machines.create_and_add(g, 1),
                    set
                );
                if candidate > best {
                    best = candidate;
                } 
            }
        }
    }
    
    set.insert(hashable, best);
    return best;
}




fn  get_max_costs(&prices: &MachinePrices) -> GemCollection<Option<u32>> {
    let mut max_costs = GemCollection::<Option<u32>>::default();
    for g in Gem::iter() {
        if g == Gem::Geode {
            continue;
        }
        let mut tmp_max = 0;
        for g2 in Gem::iter() {
            tmp_max = cmp::max(tmp_max, prices[g2][g]);       
        }
        max_costs[g] = Some(tmp_max)
    }

    max_costs
}


pub fn task1(input: &str) -> String {
    let blueprints = parse_prices(input);
    let mut val = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        let mut set = HashMap::new();
        let mut max_costs = get_max_costs(blueprint);
        let best = max_geom(
            blueprint,
            &max_costs,
            24,
            MachineInventory{ore: 0, clay: 0, obsidian: 0, geode: 0},
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
        let mut max_costs = get_max_costs(blueprint);
        let best = max_geom(
            blueprint,
            &max_costs,
            32,
            MachineInventory{ore: 0, clay: 0, obsidian: 0, geode: 0},
            MachineInventory{ore: 1, clay: 0, obsidian: 0, geode: 0},
            &mut set
        );
        println!("{}", best);
        val *= best;
    }

    val.to_string()
}



+



