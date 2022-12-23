use crate::direction::Direction::{EAST, NORTH,WEST,SOUTH};
use crate::map::Coordinate;

#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum Direction
{
    NORTH,
    SOUTH,
    WEST,
    EAST,

}

impl Direction {
    pub fn get_next(&self)->Direction{
        match self {
            NORTH => {SOUTH}
            SOUTH=> {WEST}
            EAST => {NORTH}
            WEST => {EAST}
        }
    }

    pub fn get_coords(&self,(x,y):Coordinate)->[Coordinate;3]{
        match self {
            NORTH => {[(x,y-1),(x-1,y-1),(x+1,y-1)]}
            SOUTH => {[(x,y+1),(x-1,y+1),(x+1,y+1)]}
            WEST => {[(x-1,y),(x-1,y+1),(x-1,y-1)]}
            EAST => {[(x+1,y),(x+1,y-1),(x+1,y+1)]}
        }
    }

    pub fn move_one(&self,(x,y):Coordinate)->Coordinate{
        match self {
            NORTH => {(x,y-1)}
            SOUTH => {(x,y+1)}
            WEST => {(x-1,y)}
            EAST => {(x+1,y)}
        }
    }
}