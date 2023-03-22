use std::ops::Add;
use std::collections::VecDeque;
use std::collections::HashSet;

const N: usize = 14;

pub fn benchmark_0(input: &str) -> String  {
    return "2414".to_string();
}

pub fn benchmark_1(input: &str) -> String {
    input.chars()
        .skip(N)
        .enumerate()
        .scan(input.chars().take(N).collect::<VecDeque<char>>(),|state, (i, c)| {
            state.pop_front();
            state.push_back(c);
            if state.iter().enumerate().find(|(i, &c)| state.iter().skip(*i+1).find(|&c2| *c2==c).is_some()).is_some() {
                return Some(i);
            } else {
                return None;
            };
        })
        .last()
        .unwrap()
        .add(16)
        .to_string()
}

pub fn benchmark_2(input: &str) -> String {
    let n = 14;
    let mut alphabet = [0; 128];
    let mut num_dup = 0;
    let mut i;
    for c in input.chars().take(n) {
        i = c as usize;
        alphabet[i] += 1;
        if alphabet[i] == 2 {
            num_dup += 1;
        }
    }
    let mut i1;
    let mut i2;
    let mut last_idx=0;
    for (k, (c1, c2)) in input.chars().zip(input.chars().skip(n)).enumerate() {
        i1 = c1 as usize;
        i2 = c2 as usize;
        
        alphabet[i1] -= 1;
        if alphabet[i1] == 1 {
            num_dup -= 1;
        }
        alphabet[i2] += 1;
        if alphabet[i2] == 2 {
            num_dup += 1;
        }
        if num_dup == 0 {
            last_idx = k+n+1;
            break;
        }
    }
    
    last_idx.to_string()
}


pub fn benchmark_3(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut idx = 0;
    while let Some(slice) = bytes.get(idx..idx + 14) {
        let mut state = 0u32;
        let mut pos = (slice.len() - 1) as isize;
        while pos >= 0 {
            let bit_idx = 1 << (slice[pos as usize] % 32);
            if state & bit_idx != 0 {
                break;
            } else {
                state |= bit_idx;
                pos -= 1;
            }
        }

    if pos < 0 {
            return (idx + 14).to_string();
        }
        idx += (pos + 1) as usize;
    }
    panic!("Did not find a solutuion")
}

pub fn benchmark_4(input: &str) -> String  {
    return input.as_bytes().windows(14)
        .position(|w| {
            return w.iter().collect::<HashSet<_>>().len() == 14;
        })
        .map(|x| x + 14)
        .unwrap()
        .to_string();
}



pub fn benchmark_5(input: &str) -> String {
    let n = 14;
    let mut alphabet = [0; 128];
    let mut num_dup = 0;
    let mut i;
    for ii in input.bytes().take(n) {
        i = ii as usize;
        alphabet[i] += 1;
        if alphabet[i] == 2 {
            num_dup += 1;
        }
    }
    let mut i1;
    let mut i2;
    let mut last_idx=0;
    for (k, (c1, c2)) in input.bytes().zip(input.bytes().skip(n)).enumerate() {
        i1 = c1 as usize;
        i2 = c2 as usize;
        
        alphabet[i1] -= 1;
        if alphabet[i1] == 1 {
            num_dup -= 1;
        }
        alphabet[i2] += 1;
        if alphabet[i2] == 2 {
            num_dup += 1;
        }
        if num_dup == 0 {
            last_idx = k+n+1;
            break;
        }
    }
    
    last_idx.to_string()
}

