use ndarray::{arr2, Array2, Array, Axis};

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}






fn read_matrix(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| line.chars()
             .map(|c| c.to_digit(10).unwrap() as i32)
             .collect()
            ).collect() 
}



pub fn main(input: &str) -> String {
    let read_data = read_matrix(input);
    let m = read_data.len();
    let n = read_data[0].len();
    let heights = Array::from_shape_vec( (m, n), read_data.into_iter().flatten().collect()).unwrap();
    
    let mut score = Array2::<i32>::ones((m,n));
    let mut val;
    let mut counter;
    
    for row in 0..m {
        for col in 0..n {
            val = heights[[row, col]];
            
            // to right
            counter = 0;
            for i in (col+1)..n {
                counter += 1;
                if val <= heights[[row, i]] {
                    break;
                }
            }
            score[[row, col]] *= counter;

            
            // to left
            counter = 0;
            for i in (0..col).rev() {
                counter += 1;
                if val <= heights[[row, i]] {
                    break;
                }
            }
            score[[row, col]] *= counter;

            // to down
            counter = 0;
            for i in (row+1)..m {
                counter += 1;
                if val <= heights[[i, col]] {
                    break;
                }
            }
            score[[row, col]] *= counter;
      
            // to up
            counter = 0;
            for i in (0..row).rev() {
                counter += 1;
                if val <= heights[[i, col]] {
                    break;
                }
            }
            score[[row, col]] *= counter;
      
        }
    }
    
    score.iter().max().unwrap().to_string()
}


