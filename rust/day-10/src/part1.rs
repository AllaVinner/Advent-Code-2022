


pub fn main(input: &str) {
    let mut cycles: i32 = 0;
    let mut registry: i32 = 1;
    let mut record: Vec<i32>;
    
    let mut tmp_str;
    let mut tmp_iter;

    for line in input.lines() {
        if line.starts_with("noop") {
            println!("In none line: {:?}", line);
            cycles += 1;
            continue;
        }
        cycles += 1;
        tmp_iter = line.split(" ");
        tmp_str = tmp_iter.next().unwrap();
        tmp_str = tmp_iter.next().unwrap();

        registry += tmp_str.parse::<i32>().unwrap();
        cycles += 1;

        if cycles % 20 == 0 {
            record += cycles*registry;
        }

    }

    record.read_to_string()
}



