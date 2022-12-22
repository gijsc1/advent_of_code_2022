use std::str::FromStr;
use crate::error::{dyn_parse_error, Error};
use crate::field::Field::{Open, Unmapped, Wall};

#[derive(Eq,PartialEq,Copy, Clone,Debug)]
pub enum Field{
    Open,
    Wall,
    Unmapped
}

impl FromStr for Field{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            " " => Ok(Unmapped),
            "." => Ok(Open),
            "#" => Ok(Wall),
            other => Err(dyn_parse_error(format!("Expected ' ', '.' or '#', but found '{other}'")))
        }
    }
}

impl TryFrom<char> for Field{
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ' ' => Ok(Unmapped),
            '.' => Ok(Open),
            '#' => Ok(Wall),
            other => Err(dyn_parse_error(format!("Expected ' ', '.' or '#', but found '{other}'")))
        }
    }
}