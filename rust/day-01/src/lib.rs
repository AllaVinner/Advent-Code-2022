pub fn process_part1(input: &mut String) -> String {
    let stripped = input.replace("\r", "");
    let result = stripped
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
        
    println!("{:#?}", result);
    "WORKED".to_string()
}
