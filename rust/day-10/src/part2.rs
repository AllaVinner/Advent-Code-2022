
fn eval(cycles: i32, regestry: i32) {
    if cycles % 40 == 0 {
        println!("n");
    }
    if (regestry - (cycles % 40)).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }     

}

pub fn main(input: &str) -> String {
    let mut cycles: i32 = 1;
    let mut regestry: i32 = 1;
    let mut record: i32 = 0;
    
    let mut tmp_str;
    let mut tmp_iter;

    for line in input.lines() {
        if line.starts_with("noop") {    
            eval(cycles, regestry);
            cycles += 1; 
            continue;
        }

        eval(cycles, regestry);
        cycles += 1;
        tmp_iter = line.split(" ");
        tmp_str = tmp_iter.next().unwrap();
        tmp_str = tmp_iter.next().unwrap();

        regestry += tmp_str.parse::<i32>().unwrap();
        eval(cycles, regestry);
        cycles += 1;

    }
    println!("{:?}", cycles);

    record.to_string()
}



