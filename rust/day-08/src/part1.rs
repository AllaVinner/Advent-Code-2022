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
    
    let mut vissible = Array2::<i32>::zeros((m,n));
    let mut top;
    
    // FROM LEFT
    for row in 0..m {
        top = -1;
        for col in 0..n {
            if heights[[row, col]] > top {
                top = heights[[row, col]];
                vissible[[row, col]] = 1;
            }
        }
    }
    
    // FROM RIGHT
    for row in 0..m {
        top = -1;
        for col in (0..n).rev() {
            if heights[[row, col]] > top {
                top = heights[[row, col]];
                vissible[[row, col]] = 1;
            }
        }
    }
 
    // FROM TOP
    for col in 0..n {
        top = -1;
        for row in 0..m {
            if heights[[row, col]] > top {
                top = heights[[row, col]];
                vissible[[row, col]] = 1;
            }
        }
    }

    // FROM BUTTOM
    for col in 0..n {
        top = -1;
        for row in (0..m).rev() {
            if heights[[row, col]] > top {
                top = heights[[row, col]];
                vissible[[row, col]] = 1;
            }
        }
    }
    vissible.sum().to_string()
}
