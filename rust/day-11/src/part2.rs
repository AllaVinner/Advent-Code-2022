use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alpha1, digit1, 
    }
};

use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation_str: String,
    divisible: i64,
    true_monkey: usize,
    false_monkey: usize
}



fn read_monkey(input: &str) -> Monkey {
    let mut line_iter = input.lines();

    // Empty
    line_iter.next();

    // Init items
    let mut items: VecDeque<i64> = VecDeque::new();
    for n in line_iter.next().unwrap().split("  Starting items: ").nth(1).unwrap().split(", ") {
        items.push_back(n.to_string().parse::<i64>().unwrap());
    }

    let operation_str = line_iter.next().unwrap().split("  Operation: new = old ").nth(1).unwrap().to_string();

    let divisible = line_iter.next().unwrap().split("  Test: divisible by ").nth(1).unwrap().to_string().parse::<i64>().unwrap();

    let true_monkey = line_iter.next().unwrap().split("    If true: throw to monkey ").nth(1).unwrap().to_string().parse::<usize>().unwrap();

    let false_monkey = line_iter.next().unwrap().split("    If false: throw to monkey ").nth(1).unwrap().to_string().parse::<usize>().unwrap();

    Monkey {
        items: items,
        operation_str: operation_str,
        divisible: divisible,
        true_monkey: true_monkey,
        false_monkey: false_monkey,
    }
}

fn inspect(operation_str: &str, item: i64) -> i64 {
    let mut num: i64;
    if operation_str.split(" ").nth(1).unwrap() == "old" {
        num = item;
    } else {
        num = operation_str.split(" ").nth(1).unwrap().to_string().parse::<i64>().unwrap();
    }
    if operation_str.split(" ").nth(0).unwrap() == "*" {
        num = item * num;
    } else {
        num = item + num;
    }
    
    num
}

fn test(divisible: i64, item: i64) -> bool {
    item % divisible == 0
}

fn turn(monkeys: &mut Vec<Monkey>, i: usize){
    let mut item;
    let mut to_monkey: usize;
    for _ in 0..monkeys[i].items.len() {
        item = monkeys[i].items.pop_front().unwrap();
        item = item % (9699690);
        item = inspect(& monkeys[i].operation_str, item);



        if test(monkeys[i].divisible, item) {
            to_monkey = monkeys[i].true_monkey;
        } else {
            to_monkey = monkeys[i].false_monkey;
        }

        monkeys[to_monkey].items.push_back(item);
    }

}

pub fn main(input: &str) -> String {

    let mut monkey_iter = input.split("Monkey");
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut inspections: Vec<i64> = Vec::new();
    monkey_iter.next().unwrap();
    for m in monkey_iter {
        monkeys.push(read_monkey(m));
        inspections.push(0);
    }
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            inspections[i] += monkeys[i as usize].items.len() as i64;
            turn(&mut monkeys, i);
        }
    }
    inspections.sort();
    (inspections[inspections.len()-1]* inspections[inspections.len()-2]).to_string()
}


