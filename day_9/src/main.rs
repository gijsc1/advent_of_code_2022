use std::fs;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

mod move_operation;
mod board;

//Todays goal: define an iterator.

fn main() {
    let file = fs::File::open("input.txt").expect("Error while reading file");
    let commands = BufReader::new(file).lines()
        .map(|s|s.expect("Failure to read file"))
        .map(|s| move_operation::Move::from_str(s.as_str()))
        .map(|comm| comm.expect("Failure to parse command"));

    let mut board1 = board::create_board(1,2);
    let mut board2 = board::create_board(1,10);
    // println!("{board}");
    // test_board(&mut board);
    // let testvec = vec![0,1,2,3,4];
    // let commands = parse_commands(reader).expect("Error while parsing commands");

    for command in commands{
        board1.do_move(&command);
        board2.do_move(&command);
    }
    // println!("final boardstate:\n{}",board);
    println!("answer1:{}",board1.iter().fold(0 as u32,|acc,val|acc+(val as u32)));
    println!("answer2:{}",board2.iter().fold(0 as u32,|acc,val|acc+(val as u32)));
}
