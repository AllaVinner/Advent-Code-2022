use std::collections::HashMap;
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


fn max_geom(
    machine_cost: &MachinePrices,
    remaining_time: u32, 
    mut num_ore: u32, 
    mut num_clay: u32, 
    mut num_obs: u32, 
    mut num_geo: u32,
    mut rob_ore: u32, 
    mut rob_clay: u32, 
    mut rob_obs: u32, 
    mut rob_geo: u32,
    set: &mut HashMap< (u32, u32, u32, u32, u32, u32, u32, u32, u32), u32>
) -> u32 {
    if remaining_time == 0 {
        //println!("At time 0");
        return num_geo;
    }

    let max_core = *vec![machine_cost.ore.ore, machine_cost.clay.ore, machine_cost.obsidian.ore, machine_cost.geode.ore].iter().max().unwrap();

    if rob_ore > max_core {
        rob_ore = max_core;
    }

    if rob_clay > machine_cost.obsidian.clay {
        rob_clay = machine_cost.obsidian.clay;
    }

    if rob_obs > machine_cost.geode.obsidian {
        rob_obs = machine_cost.geode.obsidian;
    }

    if num_ore > remaining_time*max_core - rob_ore*(remaining_time -1) {
        num_ore = remaining_time*max_core - rob_ore*(remaining_time -1);
    }

    if num_clay > remaining_time*machine_cost.obsidian.clay - rob_clay*(remaining_time -1) {
        num_clay = remaining_time*machine_cost.obsidian.clay - rob_clay*(remaining_time -1);
    }

    if num_obs > remaining_time*machine_cost.geode.obsidian - rob_obs*(remaining_time -1) {
        num_obs = remaining_time*machine_cost.geode.obsidian - rob_obs*(remaining_time -1);
    }

    let hashable: (u32, u32, u32, u32, u32, u32, u32, u32, u32) = (remaining_time, 
        num_ore, num_clay, num_obs, num_geo, 
        rob_ore, rob_clay, rob_obs, rob_geo
    );
    
    match set.get(&hashable) {
        Some(best) => return *best,
        None => ()
    }
    
    
    
    if num_ore >= machine_cost.geode.ore && num_obs >= machine_cost.geode.obsidian {
        //println!("buying geo");
        return max_geom(
            machine_cost, remaining_time -1, 
            num_ore - machine_cost.geode.ore + rob_ore, 
            num_clay +rob_clay, 
            num_obs - machine_cost.geode.obsidian + rob_obs, 
            num_geo + rob_geo, 
            rob_ore, rob_clay, rob_obs, rob_geo + 1,
            set
        );
    }

    let mut best;
    let mut candidate;
    //println!("wating");
    best = max_geom(
        machine_cost, remaining_time -1, 
        num_ore + rob_ore, 
        num_clay + rob_clay, 
        num_obs + rob_obs, 
        num_geo + rob_geo, 
        rob_ore, 
        rob_clay, rob_obs, rob_geo,
        set
    );
  
    if rob_obs < machine_cost.geode.obsidian {
        if num_ore >= machine_cost.obsidian.ore && num_clay >= machine_cost.obsidian.clay {
            //println!("buying obs");
            candidate = max_geom(
                machine_cost, remaining_time -1, 
                num_ore - machine_cost.obsidian.ore + rob_ore, 
                num_clay - machine_cost.obsidian.clay + rob_clay, 
                num_obs + rob_obs, 
                num_geo + rob_geo, 
                rob_ore, rob_clay, rob_obs + 1, rob_geo,
                set
            );
            if candidate > best {
                best = candidate;
            }   
        } 
    }
    

    if rob_clay < machine_cost.obsidian.clay && num_ore >= machine_cost.clay.ore {
        //println!("buying clay");
        candidate = max_geom(
            machine_cost, remaining_time -1, 
            num_ore - machine_cost.clay.ore + rob_ore, 
            num_clay + rob_clay, 
            num_obs + rob_obs, 
            num_geo + rob_geo, 
            rob_ore, rob_clay + 1, rob_obs, rob_geo,
            set
        );
        if candidate > best {
            best = candidate;
        }
    }

    if (rob_ore < machine_cost.ore.ore || 
        rob_ore < machine_cost.clay.ore || 
        rob_ore < machine_cost.obsidian.ore || 
        rob_ore < machine_cost.geode.ore) && 
        num_ore >= machine_cost.ore.ore {
        //println!("buying ore");
        candidate = max_geom(
            machine_cost, remaining_time -1, 
            num_ore - machine_cost.ore.ore + rob_ore, 
            num_clay + rob_clay, 
            num_obs + rob_obs, 
            num_geo + rob_geo, 
            rob_ore + 1, rob_clay, rob_obs, rob_geo,
            set
        );
        if candidate > best {
            best = candidate;
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
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
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
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            &mut set
        );
        println!("{}", best);
        val = val*best;
    }

    val.to_string()
}
