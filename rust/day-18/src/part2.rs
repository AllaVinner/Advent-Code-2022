use ndarray::ArrayBase;
use ndarray::Dim;
use ndarray::OwnedRepr;
use ndarray::Array3;


fn parse_line(line: &str) -> [usize;3] {
    let mut v: Vec::<usize> = line.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
    let mut t = [0, 0, 0];
    t[2] = v.pop().unwrap();
    t[1] = v.pop().unwrap();
    t[0] = v.pop().unwrap();
    t
}

fn get_neighbours(index: [usize; 3], shape: [usize; 3]) -> Vec::<[usize; 3]> {
    let mut vec =  Vec::<[usize; 3]>::new();
    let mut tmp: [usize; 3];
    for i in 0..3 {
        tmp = index;
        if tmp[i] < shape[i]-1 {
            tmp[i] = tmp[i] + 1;
            vec.push(tmp);
        }
        tmp = index;
        if 0 < tmp[i] {
            tmp[i] = tmp[i] - 1; 
            vec.push(tmp);
        }
    }
    vec
}





pub fn main(input: &str) -> String {
    let shape = [20, 20, 20];
    let mut droplet = Array3::<i32>::zeros(shape);
    for line in input.lines()  {
        t = parse_line(line);
        droplet[t] = 1;
    }

    let mut regions = Array3::<i32>::zeros(shape);
    
    "DONE".to_string()
}


