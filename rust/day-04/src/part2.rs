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
    let r2 = linesplit.next().unwrap();
    let mut r1s = r1.split("-");
    let r11: i32 = r1s.next().unwrap().parse().unwrap();
    let r12: i32 = r1s.next().unwrap().parse().unwrap();
    let mut r2s = r2.split("-");
    let r21: i32 = r2s.next().unwrap().parse().unwrap();
    let r22: i32 = r2s.next().unwrap().parse().unwrap();
    if r21 <= r12 && r12 <= r22{
        return 1;
    } else if r21 <= r11 && r11 <= r22{
        return 1;
    } else if r11 <= r22 && r22 <= r12{
        return 1;
    } else if r11 <= r21 && r21 <= r12{
        return 1;
    }
    return 0;
}











