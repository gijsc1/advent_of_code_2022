use std::fmt::{Display, Formatter};

#[derive(Clone,Copy,Eq, PartialEq)]
pub enum Field{
    EMPTY,
    ROCK,
}

impl Display for Field{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Field::EMPTY => {write!(f,".")}
            Field::ROCK => {write!(f,"#")}
        }
    }
}