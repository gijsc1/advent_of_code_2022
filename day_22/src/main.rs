use std::collections::HashMap;
use crate::facing::Facing::{EAST, NORTH, SOUTH, WEST};
use crate::facing::Rotation::{LEFT, NONE, OPOSITE, RIGHT};
use crate::file_io::get_string;
use crate::map::Map;
use crate::types::{DO_DEBUG, DO_P2, INPUT_FILE, ZoneMap};

#[path="../../shared_code/file_io.rs"]
mod file_io;
#[path="../../shared_code/error.rs"]
mod error;
mod map;
mod field;
mod facing;
mod move_operation;

#[path="../../shared_code/parsing.rs"]
mod parsing;
mod types;

fn main() {
    let mut map:Map = get_string(INPUT_FILE).parse().unwrap();
    let zonemap = if DO_DEBUG {get_test_zonemap()} else { get_real_zonemap() };
    if DO_P2{
        map.execute_plan(true,&zonemap);
        println!("answer2:{}",map.get_password());
    } else {
        map.execute_plan(false,&zonemap);
        println!("answer1:{}",map.get_password());
    }


}

///Because doing this manually twice is still faster than designing, implementing and debugging an algorithm to do it.
/// for brevity, only maps non,connected sides.
fn get_test_zonemap()->ZoneMap{
    let mut zonemap = HashMap::with_capacity(24);
    zonemap.insert(((2,0),NORTH),((0,1),OPOSITE));
    zonemap.insert(((2,0),WEST),((1,1),LEFT));
    zonemap.insert(((2,0),EAST),((3,2),OPOSITE));

    zonemap.insert(((0,1),NORTH),((2,0),OPOSITE));
    zonemap.insert(((0,1),WEST),((3,2),RIGHT));
    zonemap.insert(((0,1),SOUTH),((2,2),OPOSITE));

    zonemap.insert(((1,1),NORTH),((2,0),RIGHT));
    zonemap.insert(((1,1),SOUTH),((2,2),LEFT));

    zonemap.insert(((2,1),EAST),((3,2),RIGHT));

    zonemap.insert(((2,2),WEST),((1,1),RIGHT));
    zonemap.insert(((2,2),SOUTH),((0,1),OPOSITE));

    zonemap.insert(((3,2),NORTH),((2,1),LEFT));
    zonemap.insert(((3,2),EAST),((2,0),OPOSITE));
    zonemap.insert(((3,2),SOUTH),((0,1),LEFT));

    zonemap

}

fn get_real_zonemap()->ZoneMap{
    let mut zonemap = HashMap::with_capacity(24);
    zonemap.insert(((1,0),WEST),((0,2),OPOSITE));
    zonemap.insert(((1,0),NORTH),((0,3),RIGHT));


    zonemap.insert(((2,0),NORTH),((0,3),NONE));
    zonemap.insert(((2,0),EAST),((1,2),OPOSITE));
    zonemap.insert(((2,0),SOUTH),((1,1),RIGHT));

    zonemap.insert(((1,1),EAST),((2,0),LEFT));
    zonemap.insert(((1,1),WEST),((0,2),LEFT));


    zonemap.insert(((0,2),WEST),((1,0),OPOSITE));
    zonemap.insert(((0,2),NORTH),((1,1),RIGHT));

    zonemap.insert(((1,2),EAST),((2,0),OPOSITE));
    zonemap.insert(((1,2),SOUTH),((0,3),RIGHT));

    zonemap.insert(((0,3),WEST),((1,0),LEFT));
    zonemap.insert(((0,3),SOUTH),((2,0),NONE));
    zonemap.insert(((0,3),EAST),((1,2),LEFT));








    zonemap

}