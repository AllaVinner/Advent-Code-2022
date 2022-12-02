use std::collections::HashMap;

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn to_int(c: &char) -> i32 {
    let hm: HashMap<char, i32> = HashMap::from([
        ('A', 0),
        ('B', 1),
        ('C', 2),
        ('X', 0),
        ('Y', 1),
        ('Z', 2)
    ]);
    *hm.get(c).unwrap()
}

fn move_score(p: i32) -> i32{
    let score = [1, 2, 3];
    score[p as usize]
}


fn result_score(result: i32) -> i32{
    let score = [0, 3, 6];
    score[result as usize]
}

fn move_and_result_to_move(p1: i32, result:i32) -> i32{
    let m = [[2, 0, 1],
             [0, 1, 2],
             [1, 2, 0]];
    
    m[p1 as usize][result  as usize]
}

fn play(game: &str)-> i32 {
    let p1 = to_int(&game.chars().nth(0).unwrap());
    let result = to_int(&game.chars().nth(2).unwrap());
    let p2 = move_and_result_to_move(p1, result);
    let ms = move_score(p2);
    let rs = result_score(result);
    ms+rs
}


pub fn main(input: &str) -> String {
    let result = input
        .lines()
        .map(|game| {
            play(game)
        })
        .sum::<i32>();
    result.to_string()
}