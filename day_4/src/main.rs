mod range;
mod error;

use std::fs;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use range::Range;
use crate::error::Error;

/// Today's goals: datatypes and error handling, wich then also ended up including modules.
/// I make to excuses for the following monstrosity. I probably went way to far with
/// "It has to be possible to do this without using a for loop."
fn main() {
    let file = fs::File::open("input.txt").expect("Error while reading file");
    let reader = BufReader::new(file);
    let answers = reader.lines().
        map(|l| l.map(|s|s.split(',')
            .map(|splitres|range::Range::from_str(splitres)).collect::<Result<Vec<Range>,Error>>()))
        .map(|res1|res1.expect("Error while reading line"))
        .map(|res2|res2.expect("Error while parsing range"))
        .map(|vec|
            [
                range::either_contains(&vec[0], &vec[1]) as u16,
                (&vec[0]).overlaps(&vec[1]) as u16
            ])
        .fold([0,0],|[acc1,acc2],[v1,v2]| [acc1+v1,acc2+v2]);
    println!("answer1: {}, {}", answers[0],answers[1]);

}


