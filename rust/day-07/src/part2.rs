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
        return CommandType::LS;
    }
    if line.starts_with("$ cd ..") {
        return CommandType::UP;
    }
    if line.starts_with("$ cd") {
        return CommandType::CD{dirname: line.split(' ').nth(2).unwrap().to_string()};
    }
    if line.starts_with("dir") {
        return CommandType::DR{name: line.split(' ').nth(1).unwrap().to_string()};
    }
    
    CommandType::FL{
        name: line.split(' ').nth(1).unwrap().to_string(),
        size: line.split(' ').nth(0).unwrap().parse::<i32>().unwrap(),
    }
}

fn setup_dir<'a, I>(dir: &mut Dir, line_iter: &mut I) 
where
     I: Iterator<Item = &'a str>,
{
    let mut line;
    loop {
        line = match line_iter.next() {
            Some(l) => l,
            None => "EOF",
        };
        if line == "EOF" {
            return;
        }
        match get_command(line) {
                CommandType::UP => break,
                CommandType::CD {dirname: d} => {
                    match dir.content.get_mut(&mut d.to_string()).unwrap_or(&mut ContentType::D(Dir{ content: HashMap::new(), size: 0,})) {
                        ContentType::F(f) => (),
                        ContentType::D(d2) => setup_dir(d2, line_iter)
                    };
                    0
                },
                CommandType::LS => 0,
                CommandType::DR {name: n} => {
                    dir.content.insert(n, ContentType::D(Dir{
                        content: HashMap::new(),
                        size: 0,
                    }));
                    0
                    },
                CommandType::FL {name: n, size: s} => {
                    dir.content.insert(n, ContentType::F(File{size: s}));
                    0
                },
            };
        
    }

}

fn calc_size(dir: &mut Dir) -> i32{
    let mut sum: i32 = 0;
    for (key, value) in dir.content.iter_mut() {
        sum += match value {
            ContentType::F(f) => f.size,
            ContentType::D(d2) => calc_size(d2),
        }
    }
    dir.size = sum;
    sum
}

fn part2_task(dir: &Dir, current_best: i32, min_val: i32) -> i32 {
    let mut cb = current_best;
    if min_val <= dir.size && dir.size < current_best {
        cb = dir.size;    
    }
    let mut val;
    for (key, value) in dir.content.iter() {
        val = match value {
            ContentType::F(f) => 0 as i32,
            ContentType::D(d2) => part2_task(d2, cb, min_val),
        };
        if min_val <= val && val < current_best {
            cb = val;    
        }
    }
    cb
}

pub fn main(input: &str) -> String {
    let mut root = Dir{
        size: 0,
        content: HashMap::new()
    };
    setup_dir(&mut root, &mut input.lines());
    let total_sum = calc_size(&mut root);
    let total_space: i32 = 70000000;
    let req_space: i32 = 30000000;
    let need_to_free: i32 = req_space - (total_space - root.size);
    part2_task(&root, root.size, need_to_free).to_string()
}