use std::str::FromStr;
use crate::cave::Cave;
use crate::file_io::get_lines;
use crate::rockformation::RockFormation;

#[path="../../shared_code/file_io.rs"]
mod file_io;
#[path="../../shared_code/error.rs"]
mod errors;
mod cave;
mod rockformation;

fn main() {
    let rocks:Vec<RockFormation>  = get_lines("input.txt")
        .map(|l|RockFormation::from_str(&l).unwrap()).collect();
    let mut cave = Cave::new(rocks);
    // println!("cave:\n{}",cave);
    // for _ in 0..5{
    //     cave.drop_sand();
    // }
    // println!("cave:\n{}",cave);
    // for _ in 0..22{
    //     cave.drop_sand();
    // }
    //println!("cave:\n{}",cave);
    cave.add_floor();
    while cave.drop_sand(){};
    println!("cave:\n{}",cave);
    println!("answer:{}",cave.get_sand_count());
    println!("Hello, world!");
}
