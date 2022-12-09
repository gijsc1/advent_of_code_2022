#[path="../../shared_code/error.rs"]
mod error;
use std::str::FromStr;
use crate::move_operation::error::{dyn_parse_error, parse_error};
use crate::move_operation::Move::{Down, Left, Right, Up};

pub enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize)
}

impl Move{
    pub fn get_amount(&self)->usize{
        *match self {
            Up(n)=> n,
            Down(n)=> n,
            Left(n)=> n,
            Right(n)=> n,
        }
    }
}

impl FromStr for Move{
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((comm,amount)) = s.split_once(' '){
            let amount_val = amount.parse()?;
            return match comm {
                "U" => Ok(Up(amount_val)),
                "D" => Ok(Down(amount_val)),
                "L" => Ok(Left(amount_val)),
                "R" => Ok(Right(amount_val)),
                _ => dyn_parse_error(format!("expected U,D,L or R, found:{comm}"))
            }
        }
        return parse_error("line contained no space");
    }
}