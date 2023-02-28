






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



