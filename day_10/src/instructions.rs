use std::borrow::{Borrow, BorrowMut};
use std::iter::Map;
use std::str::FromStr;
use crate::instructions::error::parse_error;
use crate::instructions::Instruction::{Add, Noop};

#[path="../../shared_code/error.rs"]
mod error;

#[derive(Debug,Clone)]
pub enum Instruction{
    Noop,
    Add(i32)
}

impl FromStr for Instruction{
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("debug: now parsing:{}",s);
        if s == "noop"{
            // println!("debug: found noop");
            return Ok(Noop);
        }
        if let Some(("addx",val)) = s.split_once(' '){
            // println!("debug: found add");
            return Ok(Add(val.parse()?));
        }
        return parse_error("failure to parse instruction");
    }
}

#[derive(Clone)]
pub struct Processor<I>
where I: Iterator<Item=Instruction>
{
    instructions: I,
    reg: i32,
    waiting: usize,
    current_ins: Option<Instruction>
}

// impl <I: Iterator<Item=Instruction>> Processor<I> {
//
//
// }

///TODO: find out how to make this work later
// pub fn new<T:Iterator<Item=String>,I>(instructions: T) -> Processor<I>
//     where I: Iterator<Item=Instruction>{
//     return Processor{
//         instructions:instructions.map(|s| s.parse::<Instruction>().expect("failed to parse instruction")),
//         reg: 1,
//         waiting: 0,
//         current_ins: None
//     }
// }

pub fn new<I:Iterator<Item=Instruction>>(instructions: I) -> Processor<I>{
    return Processor{
        instructions,
        reg: 1,
        waiting: 0,
        current_ins: None
    }
}

impl<I:Iterator<Item=Instruction>> Iterator for Processor<I>{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // this step takes no time.
        if let Some(Add(val)) = self.current_ins{
            if self.waiting ==0{
                self.reg+=val;
                self.current_ins=None;
            }
        }

        // if self.waiting == 0 && self.current_ins.is_some(){
        //     self.reg+=*val;
        //     self.current_ins=None;
        // }

        // this step takes a cycle.
        if self.waiting>0{
            self.waiting-=1;
        } else {
            match self.instructions.next()?{
                Noop => {}
                add_ins => {
                    self.waiting=1;
                    self.current_ins = Some(add_ins);
                }
            }
        }
        // match self.current_ins.borrow() {
        //     Some(Add(val)) =>{
        //         if self.waiting==0{
        //             self.reg+=*val;
        //             self.current_ins=None;
        //             // new_reg+=val;
        //         } else {
        //             self.waiting-=1;
        //         }
        //     },
        //     None => {
        //         match self.instructions.next()?{
        //             Noop => {}
        //             add_ins => {
        //                 self.waiting=1;
        //                 self.current_ins = Some(add_ins);
        //             }
        //         }
        //     },
        //     _ => unreachable!()
        // }
        return Some(self.reg)
    }
}
