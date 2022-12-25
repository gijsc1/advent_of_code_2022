use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};
use std::str::FromStr;
use crate::error::{dyn_parse_error, Error, parse_error};
pub type InternalType = i64;

pub struct Snafu(InternalType);

impl FromStr for Snafu {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty(){
            return Err(parse_error("Tried to parse empty string"));
        }
        let values:Result<Vec<Snafu>,Self::Err> = s.chars().map(|c|c.try_into()).collect();
        let output = values?.iter().fold(Snafu(0),|acc,val|acc*5+val.0);
        Ok(output)

    }
}


impl TryFrom<char> for Snafu{
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '-' => {Ok(Snafu(-1))}
            '=' => {Ok(Snafu(-2))}
            '0' | '1' | '2' => {Ok(Snafu(value.to_digit(3).ok_or(dyn_parse_error(format!("Failed to parse {value}")))? as InternalType))}
            _ => {Err(dyn_parse_error(format!("Expected 0,1,2,-, or =, but found {value}")))}
        }
    }
}

impl <T> Mul<T> for Snafu where
    InternalType: From<T>,
{
    type Output = Snafu;

    fn mul(self, rhs: T) -> Self::Output {
        Snafu(self.0 * InternalType::from(rhs))
    }
}

impl<T> Add<T> for Snafu where
    InternalType:From<T>{
    type Output = Snafu;

    fn add(self, rhs: T) -> Self::Output {
        Snafu(self.0 + InternalType::from(rhs))
    }
}


// impl From<Snafu> for i32{
//     fn from(val: Snafu) -> Self {
//         val.0
//     }
// }

impl From<&Snafu> for InternalType{
    fn from(val: &Snafu) -> Self {
        val.0
    }
}

impl From<InternalType> for Snafu{
    fn from(val: InternalType) -> Self {
        Snafu(val)
    }
}

impl Snafu{
    pub fn get_decimal_val(&self)->InternalType{
        self.0
    }
}

impl Display for Snafu{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut rest = self.0;
        let divider = 5;
        let mut carryover = 0;
        let mut accu:String = String::new();
        while rest+carryover!=0 {
            let remainder = rest%divider;
            // print!("old number= {rest}; remainder= {remainder}; old carryover= {carryover}; ");
            let char = match remainder+carryover {
                0|1|2 => {
                    let result = char::from_digit(u32::try_from(remainder+carryover).unwrap(),3).unwrap();
                    carryover=0;
                    result
                }
                3 => {
                    carryover=1;
                    '='
                }
                4 =>{
                    carryover=1;
                    '-'
                }
                5 =>{
                    carryover=1;
                    '0'
                }
                _ => {unreachable!()}
            };
            accu.push(char);
            rest=rest/divider;
            // println!("new number = {rest}; new carryover:{carryover}; pushing {char}");

        }
        // if carryover==1{
        //     accu.push('1')
        //     // let last = accu.pop().unwrap();
        //     // match last {
        //     //     '0'=>{accu.push('1');},
        //     //     '1'=>{accu.push('2');},
        //     //     '2'=>{accu.push('=');
        //     //           accu.push('1');},
        //     //     '-'=>{accu.push('-');
        //     //           accu.push('1');},
        //     //     '='=>{accu.push('0');
        //     //           accu.push('1');}
        //     //     _ => {unreachable!()}
        //     // };
        // }
        let result:String = accu.chars().rev().collect();
        write!(f,"{}",result)
    }
}