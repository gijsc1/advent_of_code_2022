extern crate core;

use std::fs;
use std::io::{BufRead, BufReader};
use tuple::*;

/// The solution to day 2 of the advent of code.
/// Today was learn about closures day, so there are probably more closures than there should be in
/// this solution.
fn main() {
    let temp: i32 = 4;
    println!("testval {}", temp.rem_euclid(3));
    let file = fs::File::open("input.txt").expect("Error while reading file");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    // The score calculation for question 1
    let handler_1 = |oponent,you| get_score(oponent,you)+you;
    // The score calculation for question 2
    let handler_2 = |oponent, goal| match goal {1=>0,2=>3,3=>6,_=>panic!("invalid goal") }+what_to_play(oponent, goal);
    let T2(r1,r2) = lines.map(|line| line.expect("failure to parse line"))
        .map(|line| T2(handle_line(line.as_str(),handler_1),handle_line(line.as_str(),handler_2)))
        .fold(T2(0,0),std::ops::Add::add);

    println!("total score v1: {}, v2: {}",r1,r2);

}
///Takes one line from the input file, and calculates the score for that line according to the handler funtion.
fn handle_line<F: Fn(i32,i32)->i32>(line: &str, handler: F) -> i32 {
    if let [val1,val2] = line.split(" ").map(|word| get_value(word)).collect::<Vec<i32>> ()[..]{
        return handler(val1, val2);
    } else { panic!("line contained the wrong number of entries") }

}

/// Converts all the characters used to their corresponding integer values to make reasoning easier.
fn get_value(word: &str)->i32{
    match word {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => panic!("Given string cannot be parsed as value")
    }
}

/// Calculates the score for winning/drawing/losing based on what was played.
fn get_score(oponent: i32, you: i32) -> i32
{
    match (you, oponent ){
        (1,3) | (2,1) | (3,2) => 6,
        (1,1) | (2,2) | (3,3) => 3,
        (3,1) | (1,2) | (2,3) => 0,
        _ => panic!("Invalid move used for score")
    }
}

/// Calculates what you need to play to get the goal outcome given what your opponent will play.
/// This is probably more complicated than it needs to be but I wanted this to work with the build-in
/// moduls functionality, which implied that the input needs to be lowered by 1 to make it start at 0,
/// and then increased by 1 afterwards to bring the range back up to 1..3.
/// the result is nice and short but full of unreadable magic numbers.
fn what_to_play(oponent:i32,goal:i32) ->i32
{
    return 1+ (oponent + goal -3).rem_euclid(3)
}


