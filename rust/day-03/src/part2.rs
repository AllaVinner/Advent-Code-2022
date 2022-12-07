fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn main(input: &str) -> String {
    let mut lines = input.lines();
    println!("{:?}", type_of(&lines));
    let mut ans;
    let mut line1;
    let mut line2;
    let mut line3;
    
    let mut c2: char;
    let mut c2: char;
    let mut c3: char;
    loop{
        ans = lines.next();
        if ans.is_none() {
            break;
        } 
        line1 = ans.unwrap();
        line2 = lines.next().unwrap();
        line3 = lines.next().unwrap();
        for c in lin1.chars() {
            while ! in_all {
                if 
            }
        }
    }
    "Heeey".to_string()
}



pub fn get_priority(bag: &str) -> i32 {
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









