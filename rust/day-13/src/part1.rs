use nom::sequence::delimited;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::branch::alt;
use nom::IResult;
use nom::Parser;
use nom::sequence::separated_pair;
use std::cmp::Ordering;


#[derive(Debug, Eq)]
enum NestedList {
    List(Vec<NestedList>),
    Value(u32)
}

impl PartialEq for NestedList {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(l1)) => l0 == l1,
            (Self::Value(v0), Self::Value(v1)) => v0 == v1,
            (Self::List(l0), Self::Value(v1)) => l0 == &vec![NestedList::Value(*v1)],
            (Self::Value(v0), Self::List(l1)) => &vec![NestedList::Value(*v0)] == l1,
        }
    }
}


impl PartialOrd for NestedList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NestedList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (NestedList::List(a), NestedList::List(b)) => a.cmp(b),
            (NestedList::List(a), NestedList::Value(b)) => a.cmp(&vec![NestedList::Value(*b)]),
            (NestedList::Value(a), NestedList::List(b)) => vec![NestedList::Value(*a)].cmp(&b),
            (NestedList::Value(a), NestedList::Value(b)) => a.cmp(b),
        }
    }
}

fn parse_nested_list(input: &str) -> IResult<&str, NestedList> {
    alt((
         delimited(
                 tag("["),
                 separated_list0(tag(","), parse_nested_list),
                 tag("]"),)
                 .map(|l| NestedList::List(l)),
         nom::character::complete::u32
            .map(|num| NestedList::Value(num)),
         ))(input)
}



pub fn main(input: &str) -> String { 
    let mut sum = 0;

    for (i, line_pair) in input.split("\n\n").enumerate() {
        let (p1, p2) = separated_pair(parse_nested_list, tag("\n"), parse_nested_list)(line_pair).unwrap().1;
        if p1 < p2 {
            sum += i+1;
        }
    }

    sum.to_string()
}




