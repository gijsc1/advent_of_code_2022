use std::collections::HashSet;
use crate::file_io::get_string;
use crate::valley::Valley;

#[path="../../shared_code/file_io.rs"]
mod file_io;
#[path="../../shared_code/error.rs"]
mod error;
mod valley;
mod field;
mod direction;

const THERE_AND_BACK_AGAIN:bool = true;
fn main() {
    let mut valley:Valley = get_string("input.txt").parse().unwrap();
    // println!("parsed valley; width={}, heigth={}:\n{}",valley.length,valley.height,valley);
    let mut moveset = HashSet::new();
    let mut target = valley.finish;
    let mut there_and_back_again_count = 0;
    moveset.insert(valley.start);
    for i in 0..{
        valley.move_all_blizzards();
        moveset = valley.get_reachable_spaces(&moveset);
        // for space in moveset.iter(){
        //     println!("{},{} is reachable after {} moves",space.0,space.1,i+1)
        // }
        // println!("after {} steps:\n{}",i+1,valley);
        if moveset.contains(&target){
            there_and_back_again_count+=1;
            if THERE_AND_BACK_AGAIN && there_and_back_again_count==1{
                println!("found path to finish in {} moves",i+1);
                moveset.clear();
                moveset.insert(valley.finish);
                target = valley.start;
                continue;
            }
            if there_and_back_again_count==2{
                target= valley.finish;
                moveset.clear();
                moveset.insert(valley.start);
                println!("got back to start in {} moves",i+1);
                continue;
            }
            println!("found path in {} moves",i+1);
            break;
        }

    }

}
