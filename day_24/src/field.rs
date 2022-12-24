use std::fmt::{Display, Formatter};
use crate::error::{dyn_parse_error, Error};
use crate::field::Field::{BLIZZARD, EMPTY, WALL};
use crate::direction::Direction;
use crate::direction::Direction::{EAST, NORTH, SOUTH, WEST};


///This aproach is going to be more complicated because of the multiple blizzards per field thing.
/// Moving to a set based aproach and abandoning this type except for parsing because that is already done.
#[derive(Debug,Eq, PartialEq,Copy, Clone)]
pub enum Field{
    EMPTY,
    BLIZZARD(Direction),
    WALL
}

impl Field{

}


impl Display for Field{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EMPTY => {write!(f,".")}
            WALL => {write!(f,"#")}
            BLIZZARD(EAST) => {write!(f,">")}
            BLIZZARD(WEST) => {write!(f,"<")}
            BLIZZARD(NORTH) => {write!(f,"^")}
            BLIZZARD(SOUTH) => {write!(f,"v")}
        }
    }

}


impl TryFrom<char> for Field {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.'=> Ok(EMPTY),
            '#' => Ok(WALL),
            '>' => Ok(BLIZZARD(EAST)),
            '<' => Ok(BLIZZARD(WEST)),
            'v' => Ok(BLIZZARD(SOUTH)),
            '^' => Ok(BLIZZARD(NORTH)),
            other => Err(dyn_parse_error(format!("Expected '.', '#', '>','<','v' or '^' but found {other}")))
        }
    }
}