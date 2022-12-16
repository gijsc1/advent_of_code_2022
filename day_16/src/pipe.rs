use std::str::FromStr;
use crate::error::{dyn_parse_error, Error};
use crate::parsing::Parsable;
// #[path="../../shared_code/parsing.rs"]
// mod parsing;
// use crate::pipe::parsing::Parsable;


pub(crate) type Pipeid = String;
#[derive(Debug)]
pub struct Pipe{
    pub flowrate : usize,
    pub id: Pipeid,
    pub connections: Vec<Pipeid>

}

impl FromStr for Pipe
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_,((id,flowrate),connections)) = s.parse_prefix("Valve ")
            .parse_id(" ")
            .parse_prefix(" has flow rate=")
            .parse_num()
            .parse_maybe("; tunnels lead to valves ")
            .parse_maybe("; tunnel leads to valve ")
            .parse_lst(", ",|s|s.parse_id(","))?;
        Ok(Pipe{
            flowrate: flowrate as usize,
            id:id.to_string(),
            connections:connections.iter().map(|s|s.to_string()).collect(),
        })
    }
}