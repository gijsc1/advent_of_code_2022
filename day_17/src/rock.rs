use std::iter::Cycle;
use std::slice::Iter;
use crate::constants::{Coordinate, TOWERWIDTH};
use crate::field::Field;
use crate::field::Field::EMPTY;
use crate::rock::Rock::{CORNER, CROSS, FLAT, LONG, SQUARE};
use crate::tower::Tower;
use crate::vent::Direction;

//coordinate of a rock is its lowest point, leftmost point. This can be a point outside the rock
//for irregulat shapes such as the cross
#[derive(Clone,Copy,Debug)]
pub enum Rock{
//####
FLAT,
//.#.
//###
//.#.
CROSS,
//..#
//..#
//###
CORNER,
// #
// #
// #
// #
LONG,
// ##
// ##
SQUARE
}

pub fn rockfall() -> Cycle<Iter<'static, Rock>> {
    [FLAT,CROSS,CORNER,LONG,SQUARE].iter().cycle()
}

impl Rock{
    pub fn can_move(&self,tower: &Tower,direction:Direction,location:Coordinate)->bool{
        let f = match self {
            FLAT => {flat_can_move}
            CROSS => {cross_can_move}
            CORNER => {corner_can_move}
            LONG => {long_can_move}
            SQUARE => {square_can_move}
        };
        f(tower,direction,location)
    }

    pub fn can_drop(&self,tower: &Tower,location:Coordinate)->bool{
        let f = match self {
            FLAT => {flat_can_drop}
            CROSS => {cross_can_drop}
            CORNER => {corner_can_drop}
            LONG => {long_can_drop}
            SQUARE => {square_can_drop}
        };
        f(tower,location)
    }

    pub fn mark_position(&self,tower: &mut Tower,location:Coordinate){
        let f = match self {
            FLAT => {flat_mark_position}
            CROSS => {cross_mark_position}
            CORNER => {corner_mark_position}
            LONG => {long_mark_position}
            SQUARE => {square_mark_position}
        };
        f(tower,location);
    }
}

fn flat_can_move(tower: &Tower,direction:Direction,(x,y):Coordinate)->bool{
    let field = match direction {
        Direction::LEFT => tower.get_value((x-1,y)),
        Direction::RIGHT => tower.get_value((x+4,y))
    };
    field==EMPTY
}

fn flat_can_drop(tower: &Tower,(x,y):Coordinate)->bool{
     (x..x + 4).all(|nx|tower.get_value((nx,y-1))==EMPTY)
}

fn flat_mark_position(tower: &mut Tower, (x,y):Coordinate){
    for nx in x..x + 4{
        tower.set_rock((nx,y));
    }
}

fn cross_can_move(tower: &Tower,direction:Direction,(x,y):Coordinate)->bool{
    let field = match direction {
        Direction::LEFT => tower.get_value((x-1,y+1)),
        Direction::RIGHT => tower.get_value((x+3,y+1))
    };
    tower.get_value((direction+x+1,y))==EMPTY &&
        tower.get_value((direction+x+1,y+2)) == EMPTY &&
        field==EMPTY
}
fn cross_can_drop(tower: &Tower,(x,y):Coordinate)->bool{
    tower.get_value((x+1,y-1))==EMPTY&&
        tower.get_value((x,y))==EMPTY&&
        tower.get_value((x+2,y))==EMPTY
}

fn cross_mark_position(tower: &mut Tower, (x,y):Coordinate){
    for nx in x..x + 3{
        tower.set_rock((nx,y+1));
    }
    tower.set_rock((x+1,y));
    tower.set_rock((x+1,y+2));
}

fn corner_can_move(tower: &Tower,direction:Direction,(x,y):Coordinate)->bool{
    let field = match direction {
        Direction::LEFT => tower.get_value((x-1,y)),
        Direction::RIGHT => tower.get_value((x+3,y))
    };
    tower.get_value((direction+x+2,y+1))==EMPTY &&
        tower.get_value((direction+x+2,y+2)) == EMPTY &&
        field==EMPTY
}
fn corner_can_drop(tower: &Tower,(x,y):Coordinate)->bool{
    (x..x + 3).all(|nx|tower.get_value((nx,y-1))==EMPTY)
}

fn corner_mark_position(tower: &mut Tower, (x,y):Coordinate){
    for nx in x..x + 3{
        tower.set_rock((nx,y));
    }
    tower.set_rock((x+2,y+1));
    tower.set_rock((x+2,y+2));
}

fn long_can_move(tower: &Tower,direction:Direction,(x,y):Coordinate)->bool{
    tower.get_value((direction+x,y))==EMPTY &&
        tower.get_value((direction+x,y+1)) == EMPTY &&
        tower.get_value((direction+x,y+2)) == EMPTY &&
        tower.get_value((direction+x,y+3)) == EMPTY
}

fn long_can_drop(tower: &Tower,(x,y):Coordinate)->bool{
    tower.get_value((x,y-1))==EMPTY
}

fn long_mark_position(tower: &mut Tower, (x,y):Coordinate){
    for ny in y..y + 4{
        tower.set_rock((x,ny));
    }
}
fn square_can_move(tower: &Tower,direction:Direction,(x,y):Coordinate)->bool{
     match direction {
         Direction::LEFT => tower.get_value((x - 1, y)) == EMPTY && tower.get_value((x - 1, y + 1)) == EMPTY,
         Direction::RIGHT => tower.get_value((x + 2, y)) == EMPTY && tower.get_value((x + 2, y + 1)) == EMPTY
     }
}

fn square_can_drop(tower: &Tower,(x,y):Coordinate)->bool{
    (x..x + 2).all(|nx|tower.get_value((nx,y-1))==EMPTY)
}

fn square_mark_position(tower: &mut Tower, (x,y):Coordinate){
    tower.set_rock((x,y));
    tower.set_rock((x+1,y));
    tower.set_rock((x,y+1));
    tower.set_rock((x+1,y+1));
}