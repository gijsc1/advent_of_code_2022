use std::fs;
use std::io::{BufRead, BufReader, Read};

pub fn get_lines(filename: &str) -> impl Iterator<Item=String>
{
    let file = fs::File::open(filename).expect("Error while opening file");
    let lines = BufReader::new(file).lines()
        .map(|s|s.expect("Failure to read file"));
    return lines;
}

pub fn get_string(filename:&str) -> String{
    let mut file = fs::File::open(filename).expect("Error while opeing file");
    let mut result:String = String::new();
    file.read_to_string(&mut result).expect("Error while reading file");
    return result;

}