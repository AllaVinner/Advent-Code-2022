use ndarray::{arr2, Array2, Array, Axis, Dim, OwnedRepr, ArrayBase};
use std::collections::VecDeque;

fn read_heights(input: &str) -> ([usize; 2], [usize; 2], ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>) {
    let m: usize = input.lines().count();
    let n: usize = input.lines().next().unwrap().chars().count();
    let mut start_index: [usize; 2] = [0, 0];
    let mut end_index: [usize; 2] = [0, 0];
    let mut heights = Array2::<i32>::zeros((m,n));

    for (mi, line) in input.lines().enumerate() {
        for (ni, c) in line.chars().enumerate() {
            if c == 'S' {
                heights[[mi, ni]] = 'a' as i32 - 97;
                start_index = [mi, ni];
            } else if c == 'E' {
                heights[[mi, ni]] = 'z' as i32 - 97;
                end_index = [mi, ni];
            } else {
                heights[[mi, ni]] = c as i32 - 97;
            }
        }
    }
    return (start_index, end_index, heights);
}

fn get_neighbours(index: &[usize; 2], m: usize, n: usize) -> Vec<[usize; 2]> {
    let mut v: Vec<[usize; 2]> = Vec::new();
    if index[0] > 0 {
        v.push([index[0]-1,index[1]]);
    }
    if index[0] < m-1  {
        v.push([index[0]+1,index[1]]);
    }
    if index[1] > 0 {
        v.push([index[0],index[1]-1]);
    }
    if index[1] < n-1  {
        v.push([index[0],index[1]+1]);
    }
    v
}


pub fn main(input: &str) -> String {

    let (start_index, end_index, mut heights) = read_heights(&input);
    let m: usize = heights.nrows();
    let n: usize = heights.ncols();
    let mut distance = Array2::<i32>::zeros((m, n));
    let mut nodes = VecDeque::new();
    let mut current_index;
    let mut current_height = 0;
    let mut end_distance: i32 = 0;
    let mut neighbours;
    let current_distance: i32;
    let mut shortest_path: i32 = m as i32 * n as i32;
    
    for mi in 0..m {
        for ni in 0..n {
            if heights[[mi, ni]] != 0 {
                continue;
            }
            distance = 0*distance;
            nodes.push_back([mi, ni]);
            while ! nodes.is_empty() {
                current_index = nodes.pop_front().unwrap();
                current_height = heights[current_index];
                neighbours = get_neighbours(&current_index, m, n);
                for neighbour in neighbours{
                    if heights[neighbour] > current_height + 1 {
                        continue;
                    }
                    if distance[neighbour] != 0 {
                        continue
                    }
                    distance[neighbour] = distance[current_index] + 1;
                    nodes.push_back(neighbour);
                    if neighbour == end_index {
                        end_distance = distance[neighbour]
                    }
                }
                
            }
            if end_distance < shortest_path {
                shortest_path = end_distance;
            }
        }
    }
    shortest_path.to_string()
}



