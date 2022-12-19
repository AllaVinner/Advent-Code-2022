use std::collections::HashMap;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::multi::separated_list1;


pub fn main(input: &str) -> String {
    let mut dicitinary: HashMap<&str, usize> = HashMap::new();
    let mut i: usize = 0;
    for line in input.lines().enumerate() {
        dicitinary.insert(
                (delimited(tag("Valve "), alpha1, tag(" has "))(line) as IResult<&str, &str>).unwrap(),
                i as usize
        )
    }
    println!("{:?}", dicitinary);

    let mut line = input.lines().next().unwrap();
    let (mut s, name) = (delimited(tag("Valve "), alpha1, tag(" has "))(line) as IResult<&str, &str>).unwrap();
    let (mut s, rate) = (preceded(tag("flow rate="), nom::character::complete::i32)(s) as IResult<&str, i32>).unwrap();
    let (mut s, conn) = (preceded(tag("; tunnels lead to valves "), separated_list1(tag(", "), alpha1))(s) as IResult<&str, Vec::<&str>>).unwrap();
    
    println!("{:?}", conn);
    "Done".to_string()
}



