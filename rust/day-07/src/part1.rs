use std::str::Split;
use std::collections::HashMap;


#[derive(Debug)]
enum CommandType {
    UP,
    CD {dirname: String},
    LS,
    DR {name: String},
    FL {name: String, size: i32},
}

#[derive(Debug)]
enum ContentType {
    F(File),
    D(Dir),
}

#[derive(Debug)]
struct File {
    size: i32,
}

#[derive(Debug)]
struct Dir {
    content: HashMap<String, ContentType>,
    size: i32,
}



fn get_command(line: &str) -> CommandType {
    if line.starts_with("$ ls") { 
        println!("Found LS");
        return CommandType::LS;
    }
    if line.starts_with("$ cd ..") {
        println!("Found CD up");
        return CommandType::UP;
    }
    if line.starts_with("$ cd") {
        println!("Found CD in");
        return CommandType::CD{dirname: line.split(' ').nth(2).unwrap().to_string()};
    }
    if line.starts_with("dir") {
        println!("Found dir ");
        return CommandType::DR{name: line.split(' ').nth(1).unwrap().to_string()};
    }
    
    CommandType::FL{
        name: line.split(' ').nth(1).unwrap().to_string(),
        size: line.split(' ').nth(0).unwrap().parse::<i32>().unwrap(),
    }
}

fn step_in<'a, I>(dir: &mut Dir, line_iter: &mut I) 
where
     I: Iterator<Item = &'a str>,
{
    let mut line;
    for _ in 1..4 {
        line = match line_iter.next() {
            Some(l) => l,
            None => "EOF",
        };
        if line == "EOF" {
            return;
        }
        match get_command(line) {
                CommandType::UP => println!("UP"),
                CommandType::CD {dirname: d} => {
                    match dir.content.get_mut(&mut d.to_string()).unwrap() {
                        ContentType::F(f) => println!("WHAT"),
                        ContentType::D(d2) => step_in(d2, line_iter)
                    };
                    println!("CD")
                },
                CommandType::LS => println!("LS"),
                CommandType::DR {name: n} => {
                    dir.content.insert(n, ContentType::D(Dir{
                        content: HashMap::new(),
                        size: 0,
                    }));
                    println!("DR")
                    },
                CommandType::FL {name: n, size: s} => {
                    dir.content.insert(n, ContentType::F(File{size: s}));
                    println!("FL")
                },
            };
        
    }

}

pub fn main(input: &str) -> String {
    let mut root = Dir{
        size: 0,
        content: HashMap::new()
        };
    let input1 = "$ ls\ndir drblq\n133789 fjf\n";
    step_in(&mut root, &mut input1.lines());
    println!("{:?}", root);
    "DONE".to_string()
}