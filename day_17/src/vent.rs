use std::ops::Add;
use std::slice::Iter;
use std::str::FromStr;
use crate::constants::Xcoord;
use crate::error::{Error, parse_error};
use crate::vent::Direction::{LEFT, RIGHT};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    LEFT,
    RIGHT
}

impl Add<Xcoord> for Direction{
    type Output = Xcoord;

    fn add(self, rhs: Xcoord) -> Self::Output {
        match self {
            LEFT => rhs-1,
            RIGHT => rhs+1
        }
    }
}

pub struct Vents{
    pub vents:Vec<Direction>
}

impl FromStr for Vents{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vents = Vec::new();
        for c in s.chars(){
            match c {
                '>' => vents.push(RIGHT),
                '<' => vents.push(LEFT),
                _ => return Err(parse_error("Expected '<', or '>'"))
            }
        }
        Ok(Vents{vents})
    }
}