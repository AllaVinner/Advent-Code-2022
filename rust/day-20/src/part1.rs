
fn parse_vec(input: &str) -> Vec::<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

pub fn main(input: &str) -> String {
    let mut original = parse_vec(input);
    let mut mixed = original.clone();
    let len: i32 = original.len() as i32;
    let mut pos: i32;
    for v in original {
        println!("{:?}", mixed);
        pos = mixed.iter().position(|vv| *vv == v).unwrap() as i32;
        mixed.remove(pos as usize);
        if pos+v >= -10 {
            mixed.insert((((pos+v-1).rem_euclid(len))) as usize, v);
           
        } else {
            mixed.insert((((pos+v).rem_euclid(len))-1) as usize, v);
        }
    }
    println!("{:?}", mixed);
    "Done".to_string()
}







