use crate::file_io::get_lines;
use crate::instructions::{Instruction};

mod file_io;
mod instructions;

fn main() {
    let lines = get_lines("input.txt")
        .map(|l|l.parse::<Instruction>().unwrap());

    let mut processor = instructions::new(lines);

    for (val,cycle) in processor.zip((0..40).into_iter().cycle()){

        // print!("debug: val: {val},pos: {cycle} result: ");
        if (cycle-val).abs() <=1{
            print!("#");
        } else {
            print!(".")
        }
        // print!("\n");
        if cycle==39{
            println!()
        }
    }

    // println!("answer1: {answer1}");
}
