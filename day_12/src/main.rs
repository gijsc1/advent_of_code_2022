use crate::board::parse_board;
use crate::file_io::get_lines;

#[path="../../shared_code/file_io.rs"]
mod file_io;
mod board;

fn main() {
    let lines = get_lines("input.txt");
    let mut board = parse_board(lines);

    println!("answer1:{}",board.get_shortest_length());
    let answer2 = board.get_iter().fold(u16::MAX,|acc,(height,distance)|
        if *height == 0 && *distance < acc { *distance } else { acc });
    println!("answer2:{}",answer2);
    // println!("Hello, world!");
}
