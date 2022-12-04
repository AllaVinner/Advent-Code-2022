fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn main(input: &str) -> String {
    let result = input
        .lines()
        .map(|elf_pair| {
            are_overlapping(elf_pair)
        })
        .sum::<i32>();
    result.to_string()
}

pub fn are_overlapping(line: &str) -> i32 {
    let mut linesplit = line.split(",");
    let r1 = linesplit.next().unwrap();
    let r2 = inesplit.next().unwrap();
    
    30
}



fn get_priority(bag: &str) -> i32 {
    let half= bag.len() / 2;
    let comp1 = &bag[..half];
    let comp2 = &bag[half..];
    let mut num: i32 =0;
    for item1 in comp1.chars() {
        for item2 in comp2.chars() {
            if item1 == item2 {
                num = item1 as i32;
                if num > 90 {
                    return num -96;
                } else {
                    return num - 65 + 27;
                };
            };
        }
    }
    return 0;
}









