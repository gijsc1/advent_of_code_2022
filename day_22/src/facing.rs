use std::fmt::{Display, Formatter};
use crate::error::{Error, parse_error};
use crate::facing::Facing::{EAST, WEST,NORTH,SOUTH};
use crate::facing::Rotation::{LEFT,RIGHT,NONE,OPOSITE};
use crate::types::{Zone, ZONE_SIZE};

#[derive(Eq, PartialEq,Copy, Clone,Hash)]
pub enum Facing{
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

#[derive(Eq, PartialEq,Copy, Clone)]
pub enum Rotation{
    LEFT,
    RIGHT,
    OPOSITE,
    NONE
}

impl Display for Facing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NORTH => {write!(f,"north")}
            SOUTH => {write!(f,"south")}
            EAST => {write!(f,"east")}
            WEST => {write!(f,"west")}
        }
    }
}

impl Facing {
    pub fn rotate(&self,direction:Rotation)->Facing{
        match (direction,self) {
            (RIGHT, NORTH) => {EAST}
            (RIGHT, SOUTH) => {WEST}
            (RIGHT, EAST) => {SOUTH}
            (RIGHT, WEST) => {NORTH}
            (LEFT, NORTH) => {WEST}
            (LEFT, SOUTH) => {EAST}
            (LEFT, EAST) => {NORTH}
            (LEFT, WEST) => {SOUTH}
            (NONE,facing)=>{*facing}
            (OPOSITE, NORTH) => {SOUTH}
            (OPOSITE, SOUTH) => {NORTH}
            (OPOSITE, EAST) => {WEST}
            (OPOSITE, WEST) => {EAST}
        }
    }

    pub fn get_value(&self)->usize{
        match self {
            NORTH => {3}
            SOUTH => {1}
            EAST => {0}
            WEST => {2}
        }
    }

}
const MAX_ZONE_INDEX:usize = ZONE_SIZE-1;

impl Rotation {
    pub fn move_coordinate(&self,x:usize,y:usize,(oldzx,oldzy):&Zone,(newzx,newzy):&Zone)->(usize,usize){
        let normalized_x = x - oldzx*ZONE_SIZE;
        let normalized_y = y - oldzy*ZONE_SIZE;
        assert!(normalized_x<ZONE_SIZE,"original values : x:{},y:{}. new values: x:{},y:{}",x,y,normalized_x,normalized_y);
        assert!(normalized_y<ZONE_SIZE,"original values : x:{},y:{}. new values: x:{},y:{}",x,y,normalized_x,normalized_y);
        let (rotated_x,rotated_y) = match self {
            LEFT => {(normalized_y,MAX_ZONE_INDEX-normalized_x)}
            RIGHT => {(MAX_ZONE_INDEX-normalized_y,normalized_x)}
            Rotation::OPOSITE => {(MAX_ZONE_INDEX-normalized_x,MAX_ZONE_INDEX-normalized_y)}
            NONE => {(normalized_x,normalized_y)}
        };

        (rotated_x+newzx*ZONE_SIZE,rotated_y+newzy*ZONE_SIZE)
    }
}

impl TryFrom<char> for Rotation{
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(RIGHT),
            'L'=> Ok(LEFT),
            _ => Err(parse_error("failed to parse direction"))
        }
    }
}