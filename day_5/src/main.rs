
mod board;

use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use crate::board::Board;

/// Today's goal: regular expressions
/// This took so much time that this is all the documentation anyone is going to get.
fn main() {
    let file = fs::File::open("input.txt").expect("Error while reading file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.expect("Failure parsing line"));
    let numberline_regex = Regex::new(r"( \d )").unwrap();
    let mut puzzle_temp_storage = Vec::new();
    let mut matches;
    loop {
        let line = lines
            .next()
            .expect("Failed to find bottom of crate drawing");
        matches = numberline_regex.find_iter(&line).count();
        if matches>0 {
            println!("puzzle size: {matches}");
            break;
        } else {
            puzzle_temp_storage.push(line);
        }
    }

    let mut board = Board::new(matches,puzzle_temp_storage.len());
    let lineregex = Regex::new(r" ( ) |\[([[:alpha:]])\] ?").unwrap();
    for line in puzzle_temp_storage.iter().rev(){
        let row:Vec<char> = lineregex.captures_iter(line)
            .map(|value|value.get(1).or(value.get(2)))
            .map(|maybe_match| maybe_match.expect("Found a match without matching capture groups"))
            .map(|mathc|mathc.as_str().chars().nth(0).expect("Found empty match"))
            .collect();
        board.push_row(row);
    }
    lines.next();
    println!("starting board:\n{}",board);
    let mut board2 = board.clone();
    let move_regex = Regex::new(r"move (?P<amount>\d+) from (?P<from>\d) to (?P<to>\d)").unwrap();
    for line in lines{
        let matches = move_regex.captures(&line).expect("Failed to parse move operation");
        let amount:usize = matches.name("amount").unwrap().as_str().parse().expect("failed to parse amount");
        let from:usize = matches.name("from").unwrap().as_str().parse().expect("failed to parse from");
        let to:usize = matches.name("to").unwrap().as_str().parse().expect("failed to parse to");
        board.move_amount(amount,from-1,to-1);
        board2.move_multi(amount,from-1,to-1);
    }

    println!("final boardstate1:\n{}",board);
    println!("solution 1: {}",board.get_top());
    println!("***********");
    println!("final boardstate2:\n{}",board2);
    println!("solution 2: {}",board2.get_top());
    println!("done!");
}
