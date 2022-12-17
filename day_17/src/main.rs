use std::time::SystemTime;
use crate::file_io::get_string;
use crate::rock::rockfall;
use crate::tower::Tower;
use crate::vent::Vents;

mod tower;
mod field;
mod rock;
mod vent;
#[path="../../shared_code/file_io.rs"]
mod file_io;
#[path="../../shared_code/error.rs"]
mod error;
mod constants;

const ROCKCOUNT:usize = 2022;
const Q2TARGET:usize =  1000000000000;
// for count  = 1000000, res =  1594844

fn main() {
    let directions:Vents = get_string("input.txt").parse().unwrap();
    let mut tower = Tower::new();
    let rocks = rockfall().zip(0..ROCKCOUNT);
    let mut vent_iter = directions.vents.iter().cycle();
    let start = SystemTime::now();
    for (rock,_) in rocks{
        tower.add_rock(*rock,&mut vent_iter);
    }
    let end =  SystemTime::now();
    let duration = end
        .duration_since(start)
        .expect("Time went backwards");
    let answer1 = tower.extra_height+ tower.get_height()as u128;
    println!("answer1: {answer1} in {} ms",duration.as_millis());
    // println!("tower:\n{tower}");
    // println!("Hello, world!");
}
