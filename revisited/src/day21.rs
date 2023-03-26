use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Operand {
    ADD,
    SUB,
    MUL,
    DIV
}

fn left_inv(res: i64, rhs: i64, op: Operand) -> i64 {
    match op {
        Operand::ADD => res - rhs,
        Operand::SUB => res + rhs,
        Operand::MUL => res / rhs,
        Operand::DIV => res * rhs,
    }
}

fn right_inv(res: i64, lhs: i64, op: Operand) -> i64 {
    match op {
        Operand::ADD => res - lhs,
        Operand::SUB => lhs - res,
        Operand::MUL => res / lhs,
        Operand::DIV => lhs / res,
    }
}


#[derive(Debug)]
struct Reactive {
    id: String,
    lhs: String,
    rhs: String,
    operand: Operand,
    value: Option<i64>
}

fn execute(id: &str, graph: &mut Graph) -> Option<i64> {
    let mut reactive_stack: Vec<String> = Vec::new();
    if graph.get(id).unwrap().value.is_some() {
        return graph.get(id).unwrap().value;
    }
    reactive_stack.push(id.to_string());
    let mut react_id;
    let mut reactive;
    while ! reactive_stack.is_empty() {
        react_id = reactive_stack.pop().unwrap();
        reactive = graph.get(&react_id).unwrap();
        let lhs_react = graph.get(&reactive.lhs).unwrap();
        let mut lhs = match lhs_react.value {
            Some(v) => v,
            None => {
                reactive_stack.push(react_id);
                reactive_stack.push(reactive.lhs.clone());
                continue;
            }
        };

        let rhs_react = graph.get(&reactive.rhs).unwrap();
        let mut rhs = match rhs_react.value {
            Some(v) => v,
            None => {
                reactive_stack.push(react_id);
                reactive_stack.push(reactive.rhs.clone());
                continue;
            }
        };
        let mut mut_reactive = graph.get_mut(&react_id).unwrap();
        mut_reactive.value = match mut_reactive.operand {
            Operand::ADD => Some(lhs + rhs),
            Operand::SUB => Some(lhs - rhs),
            Operand::MUL => Some(lhs * rhs),
            Operand::DIV => Some(lhs / rhs),
        };
        
    }


    return graph.get(id).unwrap().value;
}



fn parse_input(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();
    for line in input.lines() {
        let (id, expr) = line.split_once(":").unwrap();
        if expr.len() <= 8 {
            // Value
            let value = expr[1..].parse::<i64>().unwrap();
            graph.insert(id.to_string(), Reactive { id: id.to_string(), 
                                                         lhs: "NONE".to_string(), 
                                                         rhs: "NONE".to_string(), 
                                                         operand: Operand::ADD,
                                                         value: Some(value)});
        } else {
            let mut expr_iter = expr.splitn(4, " ");
            expr_iter.next();
            let lhs = expr_iter.next().unwrap();
            let op = match expr_iter.next().unwrap() {
                "+" => Operand::ADD,
                "-" => Operand::SUB,
                "*" => Operand::MUL,
                "/" => Operand::DIV,
                _ => panic!("Unexpecteed operand"),
            };
            let rhs = expr_iter.next().unwrap();
            graph.insert(id.to_string(), Reactive { id: id.to_string(), 
                lhs: lhs.to_string(), 
                rhs: rhs.to_string(), 
                operand: op,
                value: None});
        }
    }
    graph
}


fn find_humn_stack(graph: &Graph) -> Vec<String> {
    let mut v = Vec::new();
    let mut has_checked_left: HashMap<String, bool> = HashMap::new();
    let mut has_checked_right: HashMap<String, bool> = HashMap::new();
    v.push("root".to_string());
    loop {
        let current = v.pop().unwrap();
        if current == "humn" {
            v.push(current.clone());
            break;
        }
        if current == "NONE" {
            continue;
        }
        if has_checked_left.get(&current).is_none() || !has_checked_left.get(&current).unwrap() {
            v.push(current.clone());
            v.push(graph.get(&current).unwrap().lhs.to_string());
            has_checked_left.insert(current.clone(), true);
            continue;
        }
        if has_checked_right.get(&current).is_none() || !has_checked_right.get(&current).unwrap() {
            v.push(current.clone());
            v.push(graph.get(&current).unwrap().rhs.to_string());
            has_checked_right.insert(current.clone(), true);
            continue;
        }
    }

    v
}

fn climb_stack(stack: Vec<String>, graph: &Graph) -> i64 {
    let root = graph.get("root").unwrap();
    let mut value = if stack.contains(&root.lhs) {
        graph.get(&root.rhs).unwrap().value.unwrap()
    } else {
        graph.get(&root.lhs).unwrap().value.unwrap()
    };
    for id in stack.iter() {
        if id == "root" {
            continue;
        }
        if id == "humn" {
            break;
        }

        let reactive = graph.get(id).unwrap();
        if stack.contains(&reactive.lhs) {
            // value  o-1 rhs= lhs
            value = left_inv(value, graph.get(&reactive.rhs).unwrap().value.unwrap(), reactive.operand);
        } else {
            value = right_inv(value, graph.get(&reactive.lhs).unwrap().value.unwrap(), reactive.operand);
        }
    }
    value
}

type Graph = HashMap<String, Reactive>;

pub fn task1(input: &str) -> String {
    let mut graph: Graph = parse_input(input);
    execute("root", &mut graph).unwrap().to_string()
}



pub fn task2(input: &str) -> String {
    let mut graph: Graph = parse_input(input);
    execute("root", &mut graph).unwrap().to_string();
    let humn_stack = find_humn_stack(&graph);
    let val = climb_stack(humn_stack, &graph);
    println!("{:?}", val);
    "AAA".to_string()
}






