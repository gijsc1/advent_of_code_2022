use std::fmt::{Display, Formatter, write};
use crate::cave::Field::Rock;
#[path="../../shared_code/layout.rs"]
mod layout;

use layout::Layout;
use crate::cave::layout::num_traits::Zero;
use crate::rockformation::RockFormation;

#[derive(Clone,PartialEq)]
enum Field{
    Rock,
    Sand,
    Air,
    Source,
}

impl Display for Field{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Field::Rock => {write!(f,"R")}
            Field::Sand => {write!(f,"S")}
            Field::Air => {write!(f,".")}
            Field::Source => {write!(f,"#")}
        }
    }
}

impl Zero for Field{
    fn zero() -> Self {
        Field::Air
    }
}

pub struct Cave{
    layout:Layout<Field>,
    sandcount:usize
}

impl Display for Cave{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.layout)
    }
}

impl Cave {
    pub fn new(rocks: Vec<RockFormation>) -> Cave{
        let mut layout = Layout::new(500,5);

        for rock in rocks
        {
            add_formation(&mut layout,rock);
        }
        layout.set_val(500,0,Field::Source);
        Cave{layout,sandcount:0}
    }

    pub fn drop_sand(&mut self)->bool{
        let (x,y) = self.layout.get_top_empty(500,0);
        if y == self.layout.get_height()-1{
            return false;
        } else if (x,y) == (500,0){
            self.sandcount+=1;
            return false;
        } else {
            self.sandcount+=1;
            self.layout.set_val(x,y,Field::Sand);
            return true;
        }
    }

    pub fn get_sand_count(&self)->usize{
        self.sandcount
    }

    pub fn add_floor(&mut self){
        let height = self.layout.get_height();
        let width = self.layout.get_width();
        //200 is just an estimate of how much space is needed, and not based on much, except that 50 was much to low.
        for x in 0..width+201{
            self.layout.set_val(x,height+1,Rock)
        }
    }
}

fn add_formation(layout:&mut Layout<Field>,rocks:RockFormation){
    for coord in rocks.get_raw().windows(2){
        let (xstart,ystart) = coord[0].as_tuple();
        let (xend,yend) = coord[1].as_tuple();

        if xstart==xend{
            for y in *(ystart.min(yend))..=*(ystart.max(yend)){
                layout.set_val(*xstart,y,Rock);
            }
        } else if ystart == yend
        {
            for x in *(xstart.min(xend))..=*(xstart.max(xend)){
                layout.set_val(x,*ystart,Rock);
            }
        }
        else{ unreachable!() }

    }
}