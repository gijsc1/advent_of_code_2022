use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::error::Error;

#[derive(Debug)]
pub struct Range {
    from: i32,
    to: i32
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (to,from) = s.split_once('-').
            ok_or(Error::LocalError {error_type: "ParseError", msg: "Expected - as range delimter" })?;

        let to_fromstr = to.parse::<i32>()?;
        let from_fromstr = from.parse::<i32>()?;

        Ok(Range { from: to_fromstr, to: from_fromstr })
    }
}

impl Display for Range{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} - {}",self.from,self.to)
    }
}

pub fn either_contains(r1:&Range,r2:&Range)->bool{
    r1.is_contained(r2) || r2.is_contained(r1)
}


impl Range{
    pub fn is_contained(&self,range:&Range)-> bool{
        self.to>=range.to && self.from<=range.from
    }

    fn overlaps_lower(&self,range: &Range)->bool{
        self.from<= range.from && self.to >= range.from
    }

    pub fn overlaps(&self,range: &Range)->bool {
        self.overlaps_lower(range) || range.overlaps_lower(&self)
    }
}