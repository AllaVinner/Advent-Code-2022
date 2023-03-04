
fn parse_char(c: char) -> u32{
    let ascii_num = c as u32;
    if ascii_num >= 96 {
        ascii_num - 96
    } else {
        ascii_num - 64 + 26
    }
}



pub fn task1(input: &str) -> String {
    input.lines()
        .map(|line| (line, line.chars().count()))
        .map(|(line, len)| line.split_at(len / 2))
        .map(|(bag1, bag2)| bag1.chars().find(|c1| bag2.find(*c1).is_some()).unwrap())
        .map(|c| parse_char(c))
        .sum::<u32>()
        .to_string()
}

//https://stackoverflow.com/questions/42134874/are-there-equivalents-to-slicechunks-windows-for-iterators-to-loop-over-pairs
pub fn task2(input: &str) -> String {
    input.lines()
        .array_chunks::<3>()
        .map(|[a,b,c]| a.chars().find(|c1| b.find(*c1).is_some() && c.find(*c1).is_some()).unwrap())
        .map(|c| parse_char(c))
        .sum::<u32>()
        .to_string()
}


