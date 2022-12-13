extern crate core;

use std::str::FromStr;
use packet::Packet;

use crate::file_io::get_lines;

#[path="../../shared_code/file_io.rs"]
mod file_io;
mod packet;
///Todays goal: figure out parser combinators, and apparently also the rust ordering system.

fn main() {
    let packets:Vec<Packet> = get_lines("input.txt")
        .filter(|l|l!="")
        .map(|l|Packet::from_str(l.as_str()).unwrap()).collect();

    let val:i32  = packets.chunks(2)

        .map(|p| p[0].cmp(&p[1]))
        .zip(1..)
        .filter(|(cmp,_)|cmp.is_lt())
        .map(|(_,i)|i)
        .sum();
    println!("answer1:{}",val);

    let div_packets = vec![Packet::from_str("[[2]]").unwrap(),Packet::from_str("[[6]]").unwrap()];
    let mut all_packets:Vec<&Packet> = packets.iter().chain(div_packets.iter()).collect();
    all_packets.sort();
    let val2  = all_packets.into_iter()
        .zip(1..)
        .filter(|(p,_)|div_packets.contains(p))
        .map(|(_,i)|i)
        .fold(1,|acc,val|acc*val);

    println!("answer2:{}",val2);

    println!("Hello, world!");
}
