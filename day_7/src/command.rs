#[path="../../shared_code/error.rs"]
mod error;

use std::iter::Peekable;
use std::str::FromStr;
use error::Error;
use crate::command::Command::{CD, HOME, LIST};
use crate::command::error::Error::{DynamicError, LocalError};
use crate::command::ListLine::{Dir, File};

#[derive(Debug)]
pub enum Command{
    CD (String),
    HOME,
    LIST(Vec<ListLine>)
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = s.strip_prefix("$ ").ok_or(
            LocalError {error_type:"parse error",msg:"expected $"}
        )?;
        // println!("debug: now parsing ::{:?}",command);
        if command == "ls"{
            return Ok(LIST(Vec::new()))
        }
        match command.split_once(' ') {
            Some(("cd","/")) => Ok(HOME),
            Some(("cd",dir)) => Ok(CD(dir.to_string())),
            Some((wrong,_)) => Err(DynamicError {error_type:"parse error",msg:format!("expected: cd found {wrong}")}),
            None => Err(LocalError {error_type:"parse error",msg:"expected space in line"})
        }

    }
}

pub fn parse_commands<T: AsRef<str>>(vals:impl Iterator<Item=T>) -> Result<Vec<Command>,Error>
{
    let mut iter = vals.peekable();
    let mut vec = Vec::new();
    while let Some(reff) = iter.next(){
        let line = reff.as_ref();
        // println!("debug: now parsing line :: {}",line);
        let mut command: Command = line.parse()?;
        match command {
            LIST(ref mut list)=> {
                parse_listlines( &mut iter,list);
            },
            _ =>{}
        };
        vec.push(command);
    }

    Ok(vec)
}


fn parse_listlines<I,T >(vals: &mut Peekable<I>, vec:&mut Vec<ListLine>)
where
    T:  AsRef<str>,
    I: Iterator<Item=T>
{
    while let Some(reff) = vals.peek(){
        let line = reff.as_ref();
        if let Ok(listline) = line.parse::<ListLine>(){
            vec.push(listline);
            vals.next();
        } else {
            return;
        };
    }
}

#[derive(Debug)]
pub enum ListLine{
    File{size:usize,name:String},
    Dir{name:String}

}

impl FromStr for ListLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some(("dir",name)) => Ok(Dir {name: name.to_string()}),
            Some((size,name)) => Ok(File {size:size.parse()?,name:name.to_string()}),
            None => Err(LocalError {error_type:"parse error",msg:"expected space in line"})
        }
    }
}