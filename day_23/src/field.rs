use std::fmt::{Display, Formatter};
use crate::error::{dyn_parse_error, Error};
use crate::field::Field::{ELF, EMPTY};
use crate::num_traits::Zero;

#[derive(Debug,Eq, PartialEq,Copy, Clone)]
pub enum Field{
    EMPTY,
    ELF
}

impl Display for Field{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::EMPTY => {write!(f,".")}
            Field::ELF => {write!(f,"#")}
        }
    }
}

impl Zero for Field{
    fn zero() -> Self {
        EMPTY
    }
}

impl TryFrom<char> for Field {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.'=> Ok(EMPTY),
            '#' => Ok(ELF),
            other => Err(dyn_parse_error(format!("Expected '.' or '#' but found {other}")))
        }
    }
}