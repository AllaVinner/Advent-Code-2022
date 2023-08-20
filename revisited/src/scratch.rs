

type Time = u32;
type Count = u32;


fn max_geom(
    machine_cost: & GemCollection<GemCollection<Count>>,
    max_costs: & GemCollection<Option<Count>>,
    remaining_time: Time, 
    mut gems: GemInventory,
    mut machines: MachineInventory,
    set: &mut HashMap< (Time, GemInventory, MachineInventory), Score>
) -> Score {
    if remaining_time == 0 {
        //println!("At time 0");
        return gems.geode;
    }

    for g in Gem::iter() {
        match max_ore_cost[g] {
            None => continue,
            Some(max_cost) => machines[g] = cmp::min(machines[g], max_cost)
        }
    }
    
    for g in Gem::iter() {
        match max_ore_cost[g] {
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
        machine_cost, remaining_time -1, 
        gems + machines,
        machines,
        set
    );

    for g in Gem::iter() {
        let max_cost = match max_costs[g] {
            None => continue,
            Some(max_cost) => max_cost
        };
        if machines[g] < max_costs[g].unwrap_or_else(continue) {
            if gems.exclusive_ge(machine_cost[g]) {
                candidate =  max_geom(
                    machine_cost, remaining_time -1, 
                    gems + machines - machine_cost[g],
                    machines.clone().add(g, 1),
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










