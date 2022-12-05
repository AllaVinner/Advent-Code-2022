use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alpha1, digit1, 
    }
}


pub fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    
}




