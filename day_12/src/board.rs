use std::array::IntoIter;
use std::iter::{Filter, FlatMap, Map};
use std::slice::Iter;
use queues::{IsQueue, Queue};

pub struct Heightmap{
    //layout pairs height with a distance value for use with path searching.
    //coords stored in row mayor order, (0,0) is top left.
    layout:Vec<Vec<(u8,u16)>>,
    start:(u8,u8),
    end:(u8,u8),
    height:u8,
    width:u8
}

impl Heightmap{
    pub fn get(&self, x:u8,y:u8) -> (u8,u16){
        self.layout[y as usize][x as usize]
    }

    fn get_mutable(&mut self, x:u8, y:u8) -> &mut (u8,u16){
         self.layout.get_mut(y as usize).unwrap().get_mut(x as usize).unwrap()
    }

    pub fn set_badness(&mut self,x:u8,y:u8,newval:u16){
        self.get_mutable(x,y).1=newval;
    }

    pub fn get_baddness(&self,x:u8,y:u8)->u16{
        self.get(x,y).1
    }

    pub fn get_height(&self,x:u8,y:u8)->u8{
        self.get(x,y).0
    }

    pub fn get_iter(&self) -> impl Iterator<Item=&(u8,u16)>{
        self.layout.iter().flat_map(|val|val.iter())
    }

    /// Checks wether some target field is reachable from a source field.
    /// If the target is outside the bounds or if it is not reachable, return false
    /// If the source is outside the bounds, also gives false
    pub fn is_reachable(&self,sourcex:u8,sourcey:u8,targetx:u8,targety:u8)->bool{
        // print!("debug: is {},{} reachable from {},{}?: ",targetx,targety,sourcex,sourcey);
        if targetx>=0 && targety>=0 && targetx<self.width && targety<self.height &&
            sourcex>=0 && sourcey>=0 && sourcex<self.width && sourcey<self.height {
            let retval = self.get_height(sourcex, sourcey)+1 >= self.get_height(targetx, targety);
            // println!("{}",retval);
            return retval;
        }
        // println!("false");
        return false;
    }

    ///Update badness value of field, if it is less bad to reach it from given source badness.
    /// returns true if update took place.
    fn update_baddness(&mut self,x:u8,y:u8,source_baddness:u16) -> bool{
        let target_baddness = self.get_baddness(x,y);
        if target_baddness> source_baddness+1{
            self.set_badness(x,y,source_baddness+1);
            return true;
        }
        return false;
    }


    ///Set the badness levels of all fields: the number of steps required to reach the end from that field.
    fn prep_pathfinding(&mut self){
        let (endx,endy) = self.end;
        self.set_badness(endx,endy,0);
        let mut workqueue: Queue<(u8,u8)> = Queue::new();
        workqueue.add(self.end).unwrap();
        while let Ok((x,y)) = workqueue.remove(){
            // println!("debug: now looking at {},{}",x,y);
            let baddness = self.get_baddness(x,y);
            for (xneighbour,yneighbour) in get_neighbours(x as i32,y as i32) {
                if self.is_reachable(xneighbour,yneighbour,x,y){
                    if self.update_baddness(xneighbour,yneighbour,baddness){
                        workqueue.add((xneighbour,yneighbour)).unwrap();
                    }
                }
            }
        }
        // println!("debug: badness map::");
        // self.debug_print_badness_map();
    }

    fn debug_print_badness_map(&self){
        for line in self.layout.iter(){
            for (_,d) in line.iter(){
                print!("{number:>0width$} ", number=d, width=8);
            }
            print!("\n");
        }
    }

    pub fn get_shortest_length(&mut self)->u16{
        self.prep_pathfinding();
        let (startx,starty) = self.start;
        self.get_baddness(startx,starty)

        //All this will find the path. but that is not required I realize way too late.
        // let (mut x,mut y) = self.start;
        // let (endx,endy) = self.end;
        // while x!=endx || y!=endy{
        //     let mut bestx = 0;
        //     let mut bexty = 0;
        //     let mut bestval = u16::MAX;
        //     for (xneighbour,yneighbour) in [(x+1,y),(x-1,y),(x,y+1),(x,y-1)].iter() {
        //         if self.is_reachable(x,y,*xneighbour,*yneighbour) {
        //             let distance = self.get_baddness(*xneighbour, *yneighbour);
        //             if distance < bestval {
        //                 bestval = distance;
        //                 bestx = *xneighbour;
        //                 bexty = *yneighbour;
        //             }
        //         }
        //     }
        //
        // }
    }
}

pub fn parse_board<T: Iterator<Item=String>>(input:T)-> Heightmap{
    let mut rows = Vec::new();
    let mut start = (0,0);
    let mut end = (0,0);
    for (y,line) in input.enumerate(){
        let mut row = Vec::new();
        for (x,val) in line.chars().enumerate(){
            let height = match val {
                'S' => {start = (x as u8,y as u8);0},
                'E' => {end = (x as u8,y as u8);25},
                c => {(c as u8) - ('a' as u8)}
            };
            row.push((height,u16::MAX));
        }
        rows.push(row);
    }
    let height = rows.len() as u8;
    let width = rows[0].len() as u8;
    return Heightmap{
        layout: rows,
        start,
        end,
        height,
        width,
    }
}

fn get_neighbours(x:i32, y:i32) -> impl Iterator<Item=(u8,u8)>{
    [(x+1,y),(x-1,y),(x,y+1),(x,y-1)].into_iter().filter(|(x,y)|*x>=0&&*y>=0).map(|(x,y)| (x as u8, y as u8))
}
