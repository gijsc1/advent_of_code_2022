use crate::file_io::get_string;
use crate::map::Map;

#[path="../../shared_code/file_io.rs"]
mod file_io;
#[path="../../shared_code/error.rs"]
mod error;
mod layout;
mod field;
mod num_traits;
mod map;
mod direction;

//Guessed too high:4246


fn main() {
    let mut map:Map = get_string("input.txt").parse().unwrap();
    println!("map:\n{map}");
    // for i in 1..11{
    //     println!("now processing iteration {i}, first direction = {:?}",map.first_direction);
    //     map.run_step();
    //     // println!("map step {i}:\n{map}");
    // }
    // println!("final map:\n{map}");
    println!("answer2: {}",map.run_simulation());

    // println!("answer1: {}",map.count_empty_squares());
    // println!("map step10:\n{map}");
}
