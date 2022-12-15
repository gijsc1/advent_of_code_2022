#[path="../../shared_code/coordinates.rs"]
mod coordinates;

use std::slice::Iter;
use std::str::FromStr;
use coordinates::Coordinate;
use crate::errors::{Error, parse_error};

pub struct RockFormation{
    path:Vec<Coordinate<usize>>
}

impl RockFormation{
    pub fn iter(&self) -> Iter<'_, Coordinate<usize>> {
        self.path.iter()
    }

    pub fn get_raw(&self)-> &Vec<Coordinate<usize>>{
        &self.path
    }
}


impl FromStr for RockFormation{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = s.split(" -> ")
            .map(|sub|sub.split_once(','))
            .map(|maybe|maybe.ok_or(parse_error("expected: ','")))
            .map(|res_coord|res_coord.and_then(|(x,y)| Ok(Coordinate::<usize>::new(x.parse()?,y.parse()?))))
            .collect::<Result<Vec<Coordinate<usize>>,Error>>()?;
        return Ok(RockFormation{path});

    }
}