
use nom::{
    IResult,
    character::complete as nom_char,
    sequence::separated_pair,
    bytes::complete::tag,
};

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32
}

fn are_overlaping(range1: &Range, range2: &Range) -> bool {
    (range2.start <= range1.start && range1.end <= range2.end) || 
       (range1.start <= range2.start && range2.end <= range1.end)
}

fn parse_input(line: &str) -> (Range, Range) {
    let parse: IResult<&str, ((u32, u32), (u32, u32))> = separated_pair(
        separated_pair(nom_char::u32, tag("-"),nom_char::u32),
        tag(","),
        separated_pair(nom_char::u32, tag("-"),nom_char::u32)
    )(line);
    
    let (_, ((start_1, end_1), (start_2, end_2))) = parse.unwrap();
    (Range{start: start_1, end: end_1}, Range{start: start_2, end: end_2})
}


pub fn task1(input: &str) -> String {
    input.lines()
        .map(|line| parse_input(line))
        .filter(|(r1, r2)| are_overlaping(r1, r2))
        .count()
        .to_string()
}


pub fn task2(input: &str) -> String {
    "ASD".to_string()
}

