use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use crate::file_io::get_lines;

#[path="../../shared_code/file_io.rs"]
mod file_io;
#[path="../../shared_code/error.rs"]
mod error;
const DOQ2:bool = true;
type ValType = i64;
type UValType = u64;

//guesses: 2516737963453 too low

fn main() {
    println!("max value ={}",ValType::MAX);
    let mut numbers:Vec<(usize,ValType)> = get_lines("input.txt")
        .map(|l| ValType::from_str(&l).expect("Error while parsing number"))
        .enumerate()
        .collect();
    if DOQ2{
        for (_,number) in numbers.iter_mut(){
            *number = *number * 811589153;
        }
    }
    mix(&mut numbers);


    for (i,val) in numbers.iter(){
        println!("{} is now at position {}",val,i);
    }
    let mut p0 = 0;
    for (i,val) in numbers.iter(){
        if *val==0{
            p0=*i;
            println!("p0 is {p0}");
            break;
        }
    }
    let t1 = (p0+1000)%numbers.len();
    let t2 = (p0+2000)%numbers.len();
    let t3 = (p0+3000)%numbers.len();
    println!("now looking for numbers at {},{} and {}",t1,t2,t3);
    let mut answer = (0,0,0);
    for (i,val) in numbers.iter(){
        if *i==t1{
            answer.0=*val;
        } else if *i==t2{
            answer.1=*val;
        } else if *i==t3{
            answer.2=*val;
        }
    }
    println!("answer1:{}+{}+{}={}",answer.0,answer.1,answer.2,answer.0+answer.1+answer.2);

    println!("Hello, world!");
}

fn mix(numbers: &mut Vec<(usize,ValType)>){
    let lenght:ValType = numbers.len() as ValType;

    // should map i start to i end.
    let mut shifts:Vec<usize> = (0..numbers.len()).collect();
    let iterations = if DOQ2 {10} else {1};
    for iter in 0..iterations {
        for (i, value) in numbers.iter() {
            let start: ValType = shifts[*i] as ValType;
            let mut end: ValType = start + *value;
            // println!("now moving {value} from pos {start} to pos {end}");
            if end >= lenght || end < 0 {
                //not sure if this line is correct
                end = end.rem_euclid(lenght - 1);
                // println!("corrected end pos = {end}");
            }
            if end == 0 {
                end = lenght - 1;
                // println!("corrected end pos = {end}");
            }
            if start <= end {
                // for (i,skipped_val) in shifts.iter_mut().enumerate(){
                for skipped_val in shifts.iter_mut() {
                    if *skipped_val as ValType > start && *skipped_val as ValType <= end {
                        // println!("moving {} back (orig pos:{i}, curr_pos:{skipped_val})",numbers[i].1);
                        *skipped_val -= 1;
                    }
                }
            } else {
                // for (i,skipped_val) in shifts.iter_mut().enumerate(){
                for skipped_val in shifts.iter_mut() {
                    if (*skipped_val as ValType) >= end && (*skipped_val as ValType) < start {
                        // println!("moving {} forward (orig pos:{i}, curr_pos:{skipped_val})",numbers[i].1);
                        *skipped_val += 1;
                    }
                }
            }
            shifts[*i] = end as usize;
        }

        // for shift in 0..numbers.len(){
        //     println!("after iteration {}, {} is at position {}",iter,numbers[shift].1,shifts[shift])
        // }
    }
        for (i, _) in numbers.iter_mut() {
            // println!("applying final shift to {} moving from {}",val,i);
            *i = shifts[*i];
            // println!(" to {}",i);
        }

}
