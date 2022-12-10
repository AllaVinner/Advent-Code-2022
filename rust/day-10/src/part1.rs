


pub fn main(input: &str) -> String {
    let mut cycles: i32 = 1;
    let mut registry: i32 = 1;
    let mut record: i32 = 0;
    
    let mut tmp_str;
    let mut tmp_iter;

    for line in input.lines() {
        if line.starts_with("noop") {
            if cycles % 40 == 20 {
                println!("1 on cycle {:?} with regestry {:?}", cycles, registry); 
                record += cycles*registry;
            }  
            cycles += 1; 
            continue;
        }

        if cycles % 40 == 20 {
            println!("2 on cycle {:?} with regestry {:?}", cycles, registry);
            record += cycles*registry;
        }
        cycles += 1;
        tmp_iter = line.split(" ");
        tmp_str = tmp_iter.next().unwrap();
        tmp_str = tmp_iter.next().unwrap();

        if cycles % 40 == 20 {
            println!("3 on cycle {:?} with regestry {:?}", cycles, registry);
            record += cycles*registry;
        }
        
        registry += tmp_str.parse::<i32>().unwrap();
        cycles += 1;

    }
    println!("{:?}", cycles);

    record.to_string()
}



