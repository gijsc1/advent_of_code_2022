use crate::file_io::get_lines;
use crate::snafu::Snafu;

#[path="../../shared_code/file_io.rs"]
mod file_io;
#[path="../../shared_code/error.rs"]
mod error;
mod snafu;

//wrong answer = 27210103880867;

fn main() {
    let values:Vec<Snafu> = get_lines("input.txt").map(|s|s.parse().unwrap()).collect();
    let sum = values.iter().fold(Snafu::from(0),|acc,val|acc+val);
    println!("total sum={} ; {}",sum.get_decimal_val(),sum);
    println!("Hello, world!");
}


