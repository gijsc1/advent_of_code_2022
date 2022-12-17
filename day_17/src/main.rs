use std::collections::HashMap;
use std::iter::Map;
use std::time::SystemTime;
use crate::constants::CacheKey;
use crate::file_io::get_string;
use crate::rock::Rock::FLAT;
use crate::rock::rockfall;
use crate::tower::{Tower, TowerHash};
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

const ROCKCOUNT:usize = 1010;
const Q2TARGET:usize =  1000000000000;
//1594842405266 + 1616

fn main() {
    let directions:Vents = get_string("input.txt").parse().unwrap();
    let mut tower = Tower::new();
    let rocks = rockfall().zip((0..5).cycle()).zip(0..ROCKCOUNT);
    let mut vent_iter = directions.vents.iter().zip((0..directions.vents.len())).cycle().peekable();
    let start = SystemTime::now();
    let mut cache = HashMap::<(usize,usize,TowerHash),(usize,usize)>::new();
    // cache.insert((0,0,tower.get_hashable_layout()),42);
    // println!("test cache, {}, {:?}",cache.get(&(0,0,tower.get_hashable_layout())).unwrap(),cache.get(&(1,0,tower.get_hashable_layout())));
    // tower.set_rock((0 ,1));
    // cache.insert((1,0,tower.get_hashable_layout()),50);
    // println!("test cache, {:?}, {:?}",cache.get(&(0,0,tower.get_hashable_layout())),cache.get(&(1,0,tower.get_hashable_layout())));
    for ((rock,rock_index),rock_count) in rocks{
        let (_,vent_index) = vent_iter.peek().unwrap();
        let key = (rock_index as usize,*vent_index,tower.get_hashable_layout());
        if cache.contains_key(&key){
            let (old_height,old_cycle) = cache.get(&key).unwrap();
            let cycle_length = rock_count-old_cycle;
            let cycle_height = tower.get_height()-old_height;
            println!("we found a cycle, at r={rock_index}, v={vent_index},length={} height={}",cycle_length,cycle_height);
            let possible_fast_forward = (ROCKCOUNT-rock_count)/cycle_length;
            let added_heigth = possible_fast_forward*cycle_height;
            let remaining_cycles = ROCKCOUNT-rock_count - (possible_fast_forward*cycle_length);
            println!("extra height from skipping cycles = {}, add this to heigth at cycle: {}",added_heigth+cycle_height,rock_count-cycle_length+remaining_cycles);
            //With the current setup, I cant easily skip cycles, as I dont think the iterators efficiently support that.
            //Instead, run the program a second time manually with the input this gives you, and add the results together.
            cache.clear();
            break;
        }else {
            cache.insert(key,(tower.get_height(),rock_count));
        }
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
