use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{Cycle, Peekable, Zip};
use std::ops::Range;
use crate::constants::{Coordinate, LEFT_SPAWN_DISTANCE, TOWER_HASH_LIMIT, TOWERWIDTH, UP_SPAWN_DISTANCE, Xcoord, Ycoord};
use crate::field::Field;
use crate::field::Field::{EMPTY, ROCK};
use crate::rock::Rock;
use crate::vent::{Direction, Vents};
use std::slice::Iter;

type Row = u8;
pub struct Tower {
    /// Field stored in row major order.
    /// (0,0) represents the bottom left corner
    layout: Vec<Row>,
    pub extra_height: u128
}

pub struct TowerHash([Row;TOWER_HASH_LIMIT]);

impl Hash for TowerHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for r in self.0.iter(){
            (*r).hash(state)
        }
        //to make different towers prefix free, as is required by the hash contract
        //u8::MAX should not be a valid row, and so should never be part of a layout.
        u8::MAX.hash(state)
    }
}

impl PartialEq<Self> for TowerHash {
    fn eq(&self, other: &Self) -> bool {
        self.0==other.0
        // self.0.zip(&other.0).all(|(r1,r2)|*r1==*r2)
    }
}

impl Eq for TowerHash{}

impl Tower{

    pub fn get_hashable_layout(&self) -> TowerHash{
        let limit = self.get_height().min(TOWER_HASH_LIMIT);
        let mut retval = [u8::MAX;TOWER_HASH_LIMIT];
        for (i,row) in self.layout[self.get_height()-limit..self.get_height()].iter().enumerate(){
            retval[i] = *row;
        }
        TowerHash(retval)
    }

    pub fn new<'a>() -> Tower{
        Tower{ layout: Vec::new(),extra_height:0 }
    }

    pub fn get_height(&self)->usize{
        self.layout.len()
    }

    pub fn get_spawnpoint(&self)-> Coordinate{
        (LEFT_SPAWN_DISTANCE as Xcoord, (UP_SPAWN_DISTANCE + self.get_height()) as Ycoord)
    }

    pub fn get_value(&self,coord:Coordinate)->Field{
        let (x,y) = coord;
        if x<0 || x>= TOWERWIDTH as isize || y<0 {
            ROCK
        } else if y >= self.get_height() as isize {
            EMPTY
        } else {
            // println!("debug: y={y}, height={}",self.get_height());
            let row =self.layout[y as usize];
            if row & (1<<x) > 0{
                ROCK
            } else {
                EMPTY
            }
        }
    }

    fn add_row(&mut self){
        self.layout.push(0);
    }

    pub fn set_rock(&mut self,(x,y):Coordinate){
        while y - self.get_height() as isize>=0{
            self.add_row();
        }
        let mut row = self.layout. get_mut(y as usize).unwrap();
        *row  = *row | 1<<x;
    }

    pub fn add_rock(&mut self, rock:Rock, vents: &mut Peekable<Cycle<Zip<Iter<Direction>, Range<usize>>>>){
        let mut location = self.get_spawnpoint();
        for (direction,_d_index) in vents{
            // println!("rock:{rock:?} is now at {},{} moving {direction:?}",location.0,location.1);
            if rock.can_move(self,*direction,location){
                location.0 = *direction+location.0;
            }
            if rock.can_drop(self,location){
                location.1-=1;
            } else {
                // println!("rock:{rock:?} is stopped at {},{}",location.0,location.1);
                rock.mark_position(self,location);
                break;
            }
        }
        // if self.get_height()> 1000000{
        //     self.try_cutoff();
        // }

    }

    pub fn find_highest_rock(&self,column:Xcoord)->Ycoord{
        (0..self.get_height()).rev()
            .find(|y| self.get_value((column, *y as Ycoord)) == ROCK)
            .unwrap_or(0) as Ycoord

    }

    /// An attempt at problem 2. This keeps the memory bounds of the tower constant while retaining the same answer.
    /// It was not nearly enough, as it does not improve the runtime.
    pub fn try_cutoff(&mut self){
        let new_floor = (0..TOWERWIDTH).map(|x|self.find_highest_rock(x as Xcoord))
            .min().unwrap_or(0);
        if new_floor>0{
            self.extra_height+=new_floor as u128;
            self.layout = self.layout[new_floor as usize..].to_vec();
            println!("threw away the {new_floor}, bottom floors to make space");
        }

    }

}

impl Display for Tower{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.layout.len()).rev(){
            write!(f,"|")?;
            for x in 0..TOWERWIDTH{
                write!(f,"{}",self.get_value((x as Xcoord, y as Ycoord)))?;
            }
            write!(f,"|\n")?;
        }
        write!(f,"+")?;
        for _ in 0..TOWERWIDTH{
            write!(f,"-")?;
        }
        write!(f,"+")
    }
}