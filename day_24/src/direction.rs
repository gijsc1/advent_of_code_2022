use std::fmt::{Display, Formatter};
use crate::direction::Direction::{EAST, NORTH, WEST, SOUTH};
use crate::valley::Coord;

#[derive(Debug,Copy, Clone,Eq, PartialEq,Hash)]
pub enum Direction
{
    NORTH,
    SOUTH,
    WEST,
    EAST,

}

impl Direction {
    pub fn move_one(&self,(x,y):Coord)->Coord{
        match self {
            NORTH => {(x,y-1)}
            SOUTH => {(x,y+1)}
            WEST => {(x-1,y)}
            EAST => {(x+1,y)}
        }
    }
}

impl Display for Direction{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NORTH => {write!(f,"^")}
            SOUTH => {write!(f,"v")}
            WEST => {write!(f,"<")}
            EAST => {write!(f,">")}
        }
    }
}