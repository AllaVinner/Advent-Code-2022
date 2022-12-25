
fn parse_vec(input: &str) -> Vec::<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

pub fn main(input: &str) -> String {
    let mut original = parse_vec(input);
    let mut mixed = original.clone();
    let len: i32 = original.len() as i32;
    let mut pos: i32;
    for v in original {
        //println!("{:?}", mixed);
        pos = mixed.iter().position(|vv| *vv == v).unwrap() as i32;
        mixed.remove(pos as usize);
        mixed.insert((((pos+v-1).rem_euclid(len-1))+1) as usize, v);
    }
    println!("{:?}", mixed);

    let zero_pos = mixed.iter().position(|v| *v == 0).unwrap() as usize;
    let v1 = mixed.get((zero_pos +1000 ) % mixed.len()).unwrap();
    let v2 = mixed.get((zero_pos +2000 ) % mixed.len()).unwrap();
    let v3 = mixed.get((zero_pos +3000 ) % mixed.len()).unwrap();
    let sum = v1+v2+v3;
    
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);
    sum.to_string()
}







