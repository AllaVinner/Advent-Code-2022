






pub fn task1(input: &str) -> String {
    input.split("\n\n")
        .map(|bag| 
            bag.lines()
               .map(|cal| cal.parse::<u32>().unwrap())
               .sum::<u32>()
        )
        .max()
        .unwrap()
        .to_string()
}


pub fn task2(input: &str) -> String {
    input.split("\n\n")
        .map(|bag| 
            bag.lines()
               .map(|cal| cal.parse::<u32>().unwrap())
               .sum::<u32>()
        )
        .fold(Vec::from([0; 3]), |mut top_three, elem| {
            match top_three.iter().position(|t| t < &elem) {
                Some(i) => {
                    top_three.insert(i, elem);
                    top_three.pop();},
                None => ()
            }
            top_three
        })
        .iter()
        .sum::<u32>()
        .to_string()
}



