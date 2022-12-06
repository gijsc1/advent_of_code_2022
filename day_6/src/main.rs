

use std::fs;
use std::io::{BufReader};
use circular_queue::CircularQueue;
use read_char::ReadIter;
use itertools::Itertools;

/// Today's goal: queues.
/// Apparantly these are not part of the stdlib for some reason and require external crates.
/// The queues crate seamed usefull, but does not implement iterator functionality,
/// so I ended up going with a specialized circular_queue crate.
/// Also, reading a file one character at a time also required an external crate.
/// So today mostly turned into finding the right crate for the job.
fn main() {
    let file = fs::File::open("input.txt").expect("Error while reading file");
    let reader = BufReader::new(file);
    let chars = ReadIter::new(reader).map(|c| c.expect("failure to read char"));
    let mut last4 = CircularQueue::with_capacity(4);
    let mut last14 = CircularQueue::with_capacity(14);
    let mut depth = 0;
    let mut done1= false;
    let mut done2 = false;
    for c in chars {
        depth += 1;
        done1 = handle_char(&mut last4, c, depth,done1);
        done2 = handle_char(&mut last14, c, depth,done2);
    }

    println!("done!");
}
/// Because cloning the chars iterator was not possible, this method abstracts over what queue
/// needs to handle the character, so that both questions can be answered with one walk through the
/// data.
fn handle_char(queu: &mut CircularQueue<char>,c: char, depth:usize, do_nothing: bool) -> bool
{
    if do_nothing
    {
        return true;
    }
    queu.push(c);
    if queu.is_full() && queu.iter().all_unique()
    {
        println!("final queue: {:?}",queu);
        println!("depth: {depth}");
        return true;
    }
    return false;
}

