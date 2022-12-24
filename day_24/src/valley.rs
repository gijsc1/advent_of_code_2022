use std::collections::{HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::direction::Direction;
use crate::error::{Error, parse_error};
use crate::field::Field;
use crate::field::Field::{BLIZZARD, EMPTY};

pub type XCoord = usize;
pub type YCoord = usize;
pub type Coord = (XCoord,YCoord);
pub type Blizzard = (Coord,Direction);

pub struct Valley {
    blizzards: Vec<Blizzard>,
    pub start: Coord,
    pub finish: Coord,
    pub height: YCoord,
    pub length: XCoord,
}

impl Valley{


    pub fn move_all_blizzards(&mut self){

        for (loc,dir) in self.blizzards.iter_mut(){
            *loc = get_new_blizzard_coord(*loc,*dir,self.length-2,self.height-2);
        }
    }

    pub fn get_reachable_spaces(&self,spaces:&HashSet<Coord>)->HashSet<Coord>{
        let mut new_spaces = HashSet::new();
        for (x,y) in spaces.iter(){
            if *y == 0 {
                new_spaces.insert((*x,*y));
                if self.blizzards.iter().all(|(loc,_)|*loc != (*x,*y+1)) {
                    new_spaces.insert((*x, *y + 1));
                }
                    continue;
            }
            if *y == self.height-1 {
                new_spaces.insert((*x,*y));
                if self.blizzards.iter().all(|(loc,_)|*loc != (*x,*y-1)) {
                    new_spaces.insert((*x, *y - 1));
                }
                continue;
            }
            for (newx,newy) in [(*x,*y),(*x+1,*y),(*x-1,*y),(*x,*y-1),(*x,*y+1)]{
                if newx>0 && newx<self.length-1 && newy>0 && newy<self.height-1 &&
                    self.blizzards.iter().all(|(loc,_)|*loc != (newx,newy)){
                        new_spaces.insert((newx,newy));
                }else if (newx,newy) == self.finish || (newx,newy) == self.start {
                    new_spaces.insert((newx,newy));
                }

            }
        }
        new_spaces
    }
}

impl FromStr for Valley {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut layout:Vec<Vec<Field>> = Vec::new();
        let mut blizzards = Vec::new();
        for (y,line) in s.lines().enumerate(){
            let mut row = Vec::new();
            for (x,char) in line.chars().enumerate(){
                let field = char.try_into()?;
                row.push(field);
                if let BLIZZARD(dir) = field {
                    blizzards.push(((x,y),dir));
                }
            }
            layout.push(row);
        }
        if layout.is_empty(){
            return Err(parse_error("parsed file is empty"));
        }
        let start = (layout[0].iter().zip(0..).find(|(val,_)|**val == EMPTY).map(|(_,y)|y).ok_or(parse_error("No start tile found of first row"))?,0);
        let finish = (layout[layout.len()-1].iter().zip(0..).find(|(val,_)|**val == EMPTY).map(|(_,y)|y).ok_or(parse_error("No end tile found on last row"))?,layout.len()-1);
        let height= layout.len();
        let length = layout[0].len();

        Ok(Valley{
               blizzards,
               start,
               finish,
               height,
               length,
           })
    }
}

impl Display for Valley{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height{
            for x in 0..self.length{
                if (x,y) == self.start{
                    write!(f,"S")?;
                } else if (x,y) == self.finish{
                    write!(f,"E")?;
                } else if x == 0 || y == 0 || x ==self.length-1 || y == self.height-1{
                    write!(f,"#")?;
                } else {
                    let count = self.blizzards.iter().filter(|(loc,_dir)| *loc == (x,y)).count();
                    if count == 0{
                        write!(f,".")?;
                    } else if count  == 1 {
                        write!(f,"{}",self.blizzards.iter().find(|(loc,_dir)|*loc==(x,y)).unwrap().1)?;

                    } else {
                        write!(f,"{}",count)?;
                    }
                }

            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn get_new_blizzard_coord(start:Coord,dir:Direction,max_x:XCoord,max_y:YCoord)->Coord{
    let (x,y) = dir.move_one(start);
    if x == 0{
        // println!("wrapping around from {},{} to {},{}",start.0,start.1,max_x,y);
        (max_x,y)
    } else if x==max_x+1 {
        (1,y)
    } else if y == 0 {
        (x,max_y)
    } else if y==max_y+1 {
        (x,1)
    } else {
        (x,y)
    }
}