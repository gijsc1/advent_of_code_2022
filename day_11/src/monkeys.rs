use std::collections::{HashMap};
use std::ops::{Div, Mul, Rem, SubAssign};
use std::process::Output;
use std::str::FromStr;
use crate::monkeys::error::{Error, parse_error};
use crate::monkeys::Operation::{Add, Mult, Square};
#[path="../../shared_code/error.rs"]
mod error;
use queues::*;
use std::ops::Add as Addtrait;
use num_traits::{Pow, Zero};
enum Operation{
    Add(i32),
    Mult(i32),
    Square
}

impl Operation{
    fn apply<T>(&self, input:T) ->T
    where T: Pow<u16,Output=T> + Mul<Output=T> + Addtrait<Output=T> +From<i32> {
        match self {
            Add(v) =>{T::from(*v)+(input)}
            Mult(v) =>{T::from(*v)*(input)}
            Square=>{(input).pow(2)}
        }
    }
}

impl FromStr for Operation{
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let important_bit = s.strip_prefix("  Operation: new = old ")
            .ok_or(parse_error("Operation line malformed "))?;
        // println!("debug: now parsing:{}",important_bit);
        match important_bit.split_once(' ') {
            Some(("+", "old")) => {Ok(Mult(2))},
            Some(("*", "old")) => {Ok(Square)}
            Some(("+",val))=>{Ok(Add(val.parse()?))},
            Some(("*",val))=>{Ok(Mult(val.parse()?))},
            _ => Err(parse_error("expected '+ ' or '* '"))?
        }
    }
}

pub struct Monkey{
    name:i32,
    operation:Operation,
    testval:i32,
    truetarget:i32,
    falsetarget:i32,
    throwcount:usize
}

impl Monkey {
    pub fn get_name(&self)->i32
    {
        self.name
    }

    pub fn get_divisor(&self)->i32{self.testval}

    ///calculates (newval,target_monkey) for a given monkey item combo
    pub fn handle_trow<T>(&mut self,item:T,lcm:T)-> (T,i32)
        where T: Pow<u16,Output=T> + Mul<Output=T> + Addtrait<Output=T> +From<i32> + Rem<T, Output=T> + Copy + PartialEq + Zero + PartialOrd + SubAssign{
        let mut new_val = self.operation.apply(item);
        while new_val>lcm {
            new_val-=lcm;
        }
            // no more for part 2
            // /3;
        let target = if (new_val.clone())%T::from(self.testval)==T::zero(){
            self.truetarget
        } else {
            self.falsetarget
        };
        self.throwcount+=1;
        return (new_val,target)
    }

    pub fn get_throw_count(&self)->usize{
        return self.throwcount;
    }
}

pub fn parse_monkey<T>(s: &str,vec:&mut Vec<Monkey>,item_map:&mut HashMap<i32,Queue<T>>) -> Result<(),Error> where
T:From<i32> + Clone{
    let (line1,rest)= s.split_once(":\n").ok_or(parse_error("expected :"))?;
    let name:i32 = line1.strip_prefix("Monkey ").ok_or(parse_error("expected 'Monkey'"))?.parse()?;
    let (line2,rest2)= rest.split_once("\n").ok_or(parse_error("expected \n"))?;
    // println!("debug: now parsing:\n{}",line2);
    let values = line2.strip_prefix("  Starting items: ").ok_or(parse_error("expected: Starting items:"))?;
    let mut items:Queue<T> = Queue::new();
    for value in values.split(", "){
        let val:i32 = value.parse()?;
        items.add(T::from(val)).unwrap();
    }
    item_map.insert(name,items);
    let (line3,rest3)= rest2.split_once("\n").ok_or(parse_error("expected \n"))?;
    let operation:Operation = line3.parse()?;
    let (line4,rest4)= rest3.split_once("\n").ok_or(parse_error("expected \n"))?;
    let div_val:i32 = line4.strip_prefix("  Test: divisible by ").ok_or(parse_error("expected: divisible by:"))?.parse()?;
    let (line5,line6)= rest4.split_once("\n").ok_or(parse_error("expected \n"))?;
    let true_target:i32 = line5.strip_prefix("    If true: throw to monkey ").ok_or(parse_error("expected: If true: throw to monkey "))?.parse()?;
    // println!("debug: now parsing:\n{}",rest5);
    let false_target:i32 = line6.strip_prefix("    If false: throw to monkey ").ok_or(parse_error("expected: If false: throw to monkey "))?.parse()?;
    vec.push(Monkey{
        name,
        operation,
        testval: div_val,
        truetarget: true_target,
        falsetarget: false_target,
        throwcount: 0,
    });
    Ok(())
}