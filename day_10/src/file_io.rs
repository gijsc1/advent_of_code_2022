use std::fs;
use std::io::{BufRead, BufReader};

pub fn get_lines(filename: &str) -> impl Iterator<Item=String>
{
    let file = fs::File::open(filename).expect("Error while reading file");
    let lines = BufReader::new(file).lines()
        .map(|s|s.expect("Failure to read file"));
    return lines;
}