


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


fn max_geom(remaining_time: i32, num_ore: i32, num_clay: i32, num_obs: i32, num_geo: i32,
            rob_ore: i32, rob_clay: i32, rob_obs: i32, 
            rob_geo: i32) -> i32 {
    if n 

    23
}

pub fn task1(input: &str) -> String {
    let blueprints = parse_prices(input);
    for blueprint in blueprints {
        max_geom()
    }
    "too".to_string()
}



pub fn task2(input: &str) -> String {
    "aa".to_string()
}
