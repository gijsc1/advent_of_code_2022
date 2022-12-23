use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::direction::Direction;
use crate::direction::Direction::NORTH;
use crate::error::Error;
use crate::field::Field;
use crate::field::Field::{ELF, EMPTY};
use crate::layout::Layout;

pub type YCoord = isize;
pub type XCoord = isize;
pub type Coordinate = (XCoord,YCoord);


pub struct Map{
    layout: Layout<Field>,
    northmost:YCoord,
    southmost:YCoord,
    westmost:XCoord,
    eastmost:XCoord,
    pub first_direction:Direction
}

impl Map{
    pub fn run_simulation(&mut self)->usize{
        let mut count = 1;
        while self.run_step() {
           count+=1;
        }
        count
    }

    pub fn run_step(&mut self) ->bool{
        //update bounds so that they will not shift during gathering of coordinates
        if self.northmost == 0{
            self.layout.add_space_above(1);
            self.northmost+=1;
            self.southmost+=1;
        }
        if self.westmost == 0{
            self.layout.add_space_left(1);
            self.westmost+=1;
            self.eastmost+=1;
        }
        if self.southmost == (self.layout.get_height() - 1) as isize {
            self.layout.add_space_below(1);
        }
        if self.eastmost== (self.layout.get_width()-1) as isize{
            self.layout.add_space_right(1);
        }
        // println!("Boundries found between x: {},{} and y:{},{}",self.eastmost,self.westmost,self.northmost,self.southmost);
        let (moves,blocked_targets) = self.gather_move_proposals();
        let mut move_count = 0;
        for (source,target) in moves{
            if blocked_targets.contains(&target){
                // println!("blocking move to {},{}",target.0,target.1);
                continue;
            }
            // println!("now processing move {},{} to {},{}",source.0,source.1,target.0,target.1);
            move_count+=1;
            self.do_move(source,target);
        }
        self.first_direction = self.first_direction.get_next();
        //might be wrong, because it does not think about only blocked moves.
        return move_count!=0;

    }
    /// Gathers all the move proposals for all the elfs.
    /// returns non-overlapping moves from source to target coordinate,
    /// and a set of target squares to which moves should not be executed.
    fn gather_move_proposals(&self)-> (HashMap<Coordinate,Coordinate>,HashSet<Coordinate>){
        let mut move_map = HashMap::new();
        //For collecting all coordinates that are already claimed.
        let mut coordset =  HashSet::new();
        //For collecting all coordinates that are claimed at least twice.
        let mut blocked_set = HashSet::new();
        for (field,(x,y)) in self.layout.iter_coords().filter(|(f,_)|**f==ELF){
            // println!("now gathering move proposel for elf at {},{}",x,y);
           if get_surroundings((x as XCoord,y as YCoord)).iter().all(|(nearx,neary)|*self.layout.unsafe_get((*nearx).try_into().unwrap(),(*neary).try_into().unwrap()).unwrap() == EMPTY){
               continue;
           }
            let mut direction = self.first_direction;
            for _ in 0..4{
                if direction.get_coords((x as XCoord,y as YCoord)).iter().all(|(nearx,neary)|{
                    let val = self.layout.unsafe_get((*nearx).try_into().unwrap(),(*neary).try_into().unwrap()).unwrap();
                    // println!("\t now checking field {},{}. result = {}",nearx,neary,*val);
                    return *val==EMPTY;
                }){
                    let target_loc = direction.move_one((x as XCoord,y as YCoord));
                    // println!("elf will try to move {direction:?} to {},{}",target_loc.0,target_loc.1);
                    if coordset.insert(target_loc){
                        move_map.insert((x as XCoord,y as YCoord),target_loc);
                    } else {
                        blocked_set.insert(target_loc);
                        // println!("elf will try to move {direction:?} to {},{}",target_loc.0,target_loc.1);
                        // println!("move is blocked by other elf");
                    }
                    break;
                }
                direction = direction.get_next();
            }
        }


        (move_map,blocked_set)
    }

    fn do_move(&mut self,(sourcex,sourcey):Coordinate,(targetx,targety):Coordinate){
        if targetx < self.westmost{
            self.westmost = targetx
        } else if targetx > self.eastmost{
            self.eastmost = targetx
        }
        if targety < self.northmost{
            self.northmost = targety;
        } else if targety > self.southmost{
            self.southmost = targety
        }
        self.layout.set_val(sourcex as usize,sourcey as usize,EMPTY);
        self.layout.set_val(targetx as usize,targety as usize,ELF);
    }

    pub fn count_empty_squares(&mut self) -> usize {
        //Because the code updating these only checks if they grow, not if they shrink
        self.initialize_boundries();
        println!("counting between {},{} and {},{}",self.westmost,self.northmost,self.eastmost,self.southmost);
        let mut count = 0;
        for x in self.westmost..=self.eastmost{
            for y in self.northmost..=self.southmost{
                if *self.layout.unsafe_get(x as usize,y as usize).unwrap() == EMPTY{
                    count+=1;
                }
            }
        }
        count
    }

    fn initialize_boundries(&mut self){
        'outer:for x in 0..self.layout.get_width() {
            for y in 0..self.layout.get_height(){
                if *self.layout.unsafe_get(x,y).unwrap()==ELF{
                    self.westmost = x as XCoord;
                    println!("found west border at {},{}",x,y);
                    break 'outer;

                }
            }
        }

        'outer: for x in (0..self.layout.get_width()).rev() {
            for y in 0..self.layout.get_height(){
                if *self.layout.unsafe_get(x,y).unwrap()==ELF{
                    self.eastmost = x as XCoord;
                    println!("found east border at {},{}",x,y);
                    break 'outer;
                }
            }
        }

        'outer: for y in 0..self.layout.get_height(){
            for x in 0..self.layout.get_width() {
                if *self.layout.unsafe_get(x,y).unwrap()==ELF{
                    println!("found north border at {},{}",x,y);
                    self.northmost = y as YCoord;
                    break 'outer;
                }
            }
        }

        'outer: for y in (0..self.layout.get_height()).rev(){
            for x in 0..self.layout.get_width() {
                if *self.layout.unsafe_get(x,y).unwrap()==ELF{
                    self.southmost = y as YCoord;
                    println!("found south border at {},{}",x,y);
                    break 'outer;
                }
            }
        }
    }
}

impl FromStr for Map{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let mut board = Vec::with_capacity(height);
        let length = s.lines().next().unwrap().chars().count();
        for line in s.lines(){
            let mut row = Vec::with_capacity(length);
            for c in line.chars(){
                row.push(c.try_into()?);
            }
            board.push(row);
        }
        let mut result = Map{
            layout: board.into(),
            northmost: 0,
            southmost: 0,
            westmost: 0,
            eastmost: 0,
            first_direction:NORTH
        };
        result.initialize_boundries();
        println!("parsed map. Boundries found between x: {},{} and y{},{}",result.eastmost,result.westmost,result.northmost,result.southmost);
        Ok(result)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.layout.get_raw_string())
    }
}

fn get_surroundings((x,y):Coordinate)->[Coordinate;8]{
    [(x-1,y-1),(x,y-1),(x+1,y-1),(x-1,y),(x+1,y),(x-1,y+1),(x,y+1),(x+1,y+1)]
}