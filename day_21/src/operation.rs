use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::error::{dyn_parse_error, Error};
use crate::operation::Operation::{App, Val};
use crate::operation::Operator::{ADD, DIV, MIN, MUL};
use crate::types::{MonkeyID, ONE, ValType, ZERO};
use crate::parsing::{Parsable};

pub enum Operator{
    ADD,
    MUL,
    DIV,
    MIN
}

impl Operator{
    pub fn eval(&self,lhs:ValType,rhs:ValType)->ValType{
        match self {
            ADD => {lhs+rhs}
            MUL => {lhs*rhs}
            DIV => {if rhs == ZERO {unreachable!()} else{lhs/rhs}}
            MIN =>{lhs-rhs}
        }
    }

    pub fn right_inverse(&self,output:ValType,rhs:ValType)->ValType{
        match self {
            ADD => {output-rhs}
            MUL => {if rhs == ZERO {unreachable!()} else {output/rhs}}
            DIV => {if rhs == ZERO {unreachable!()} else {output*rhs}}
            MIN => {output+rhs}
        }
    }

    pub fn left_inverse(&self,output:ValType,lhs:ValType)->ValType{
        match self {
            ADD => {output-lhs}
            MUL => {if lhs == ZERO {unreachable!()} else {output/lhs}}
            DIV => {if output == ZERO {unreachable!()} else {lhs/output}}
            MIN => {-ONE*(output-lhs)}
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            ADD => {write!(f,"+")}
            MUL => {write!(f,"*")}
            DIV => {write!(f,"/")}
            MIN => {write!(f,"-")}
        }
    }
}

impl FromStr for Operator{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+"=> Ok(ADD),
            "*"=> Ok(MUL),
            "/"=> Ok(DIV),
            "-"=> Ok(MIN),
            _ => Err(dyn_parse_error(format!("expected +, / or *, found '{s}'")))
        }
    }
}

pub enum Operation{
    Val(ValType),
    App(Operator,MonkeyID,MonkeyID),
    VAR
}
pub struct Monkey(MonkeyID,Operation);

impl From<Monkey> for (MonkeyID,Operation) {
    fn from(Monkey(id,op): Monkey) -> Self {
        (id,op)
    }
}

pub fn parse_op_line(line:&str)->Result<Monkey,Error>{
    let (rem,id) = line.parse_id(":").parse_prefix(": ")?;
    if let Ok((_,val)) = rem.parse_num(){
        Ok(Monkey(id.to_string(),Val(val.into())))
    } else {
        let (_,((lhs,op),rhs)) = rem.parse_id(" ").parse_prefix(" ").parse_id(" ").parse_prefix(" ").parse_id(" ")?;
        Ok(Monkey(id.to_string(),App(op.parse()?,lhs.to_string(),rhs.to_string())))
    }

}