use std::collections::HashMap;

#[derive(Debug)]
enum Operand {
    ADD,
    SUB,
    MUL,
    DIV
}

#[derive(Debug)]
struct Reactive {
    id: String,
    lhs: String,
    rhs: String,
    operand: Operand,
    value: Option<i32>
}



fn parse_input(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();
    for line in input.lines() {
        let (id, expr) = line.split_once(":").unwrap();
        if expr.len() <= 3 {
            // Value
            let value = expr[1..].parse::<i32>().unwrap();
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

type Graph = HashMap<String, Reactive>;

pub fn task1(input: &str) -> String {
    let mut graph: Graph = parse_input(input);
    println!("{:?}", graph);

    "AAA".to_string()
}



pub fn task2(input: &str) -> String {
    "AAA".to_string()
}






