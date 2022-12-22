use ndarray::ArrayBase;
use ndarray::Dim;
use ndarray::OwnedRepr;
use ndarray::Array3;


fn parse_droplet(input: &str) ->  ArrayBase<OwnedRepr<i32>, Dim<[usize; 3]>> {
    let mut droplet = Array3::<i32>::zeros([25, 25,25]);
    for line in input.lines() {
        let mut v: Vec::<usize> = line.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
        let mut t = [0, 0, 0];
        t[2] = v.pop().unwrap()+1;
        t[1] = v.pop().unwrap()+1;
        t[0] = v.pop().unwrap()+1;
        droplet[t] = 1;
    }droplet
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

fn get_reach(start: [usize; 3], grid: & ArrayBase<OwnedRepr<i32>, Dim<[usize; 3]>>) -> ArrayBase<OwnedRepr<i32>, Dim<[usize; 3]>> {
    let shape: [usize;3]= grid.shape().try_into().unwrap();
    let mut reach = Array3::<i32>::zeros(shape);
    let mut index_stack = Vec::<[usize; 3]>::new();
    let mut index;
    reach[start] = 1;
    index_stack.push(start);
    while(!index_stack.is_empty()) {
        index = index_stack.pop().unwrap();
        for n in get_neighbours(index, grid.shape().try_into().unwrap()) {
            if grid[n] == 1 {
                continue;
            }
            if reach[n] == 1 {
                continue;
            }
            
            reach[n] = 1;
            index_stack.push(n);
        }
    }
    reach
}

fn count_sides(reach: &ArrayBase<OwnedRepr<i32>, Dim<[usize; 3]>>, grid: &ArrayBase<OwnedRepr<i32>, Dim<[usize; 3]>>) -> i32 {
    let shape: [usize;3]= grid.shape().try_into().unwrap();
    let mut counter = 0;
    for x in 0..shape[0] {
        for y in 0..shape[1] {
            for z in 0..shape[2] {
                if reach[[x,y,z]] == 0 {
                    continue;
                }
                for n in get_neighbours([x,y,z], shape) {
                    if grid[n] == 1 {
                        counter += 1;
                    }
                }
            }
        }
    }
    counter
}


pub fn main(input: &str) -> String {
    let droplet = parse_droplet(input);
    let start = [0,0,0];    
    let reach = get_reach(start, &droplet);
    let sides = count_sides(&reach, &droplet);
    sides.to_string()
}


