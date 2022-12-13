use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use glue::combinators::structures::*;
use glue::prelude::{ digit, find_any, find_separated, is, MapParserResult, Parser, take};

use crate::packet::error::Error;
use crate::packet::Packet::{List, Val};

#[path="../../shared_code/error.rs"]
mod error;

#[derive(Debug)]
pub(crate) enum Packet{
    Val(i32),
    List(Vec<Packet>)
}

impl Display for Packet{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Val(v) => {write!(f,"{v}")}
            List(l) => {
                write!(f,"[")?;
                let len = l.len();
                for (i,p) in l.iter().enumerate(){
                    write!(f,"{p}")?;
                    if i!=len{
                        write!(f,",")?;
                    }
                }
                write!(f,"]")
            }
        }
    }
}

//several definitions that do nothing, which are aparantly needed.
impl Eq for Packet {}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
// The actual comparison logic.
impl Ord for Packet{
    fn cmp(&self, other: &Self) -> Ordering {
        match (self,other) {
            (Val(s), Val(o)) => {s.cmp(o)}
            (Val(x),List(_)) => {
                List(vec![Val(*x)]).cmp(other)
            }
            (List(self_packets),List(other_packets)) => {
                let cmp_res = self_packets.iter().zip(other_packets.iter())
                    .map(|(self_sub,other_sub)|self_sub.cmp(other_sub))
                    .find(|p|!p.is_eq());
                match cmp_res {
                    None => {self_packets.len().cmp(&other_packets.len())}
                    Some(res) => {res}
                }
            }
            (List(_),Val(_))=>{
                other.cmp(self).reverse()
            }
        }
    }
}

fn parse_val_packet<'a>() -> impl Parser<'a,Packet>{
    move |ctx| {
        take(1..,is(digit)).parse(ctx).map_result(|res|Val(i32::from_str(res).unwrap()))

        // .map_result(|token| Token::Identifier(token))
    }
}

fn parse_list_packet<'a>() -> impl Parser<'a, Packet> {
    move |ctx| {
        delimited(is('[')
                  ,find_separated(0..,parse_packet(),is(','))

                  ,is(']')).parse(ctx).map_result(|l|List(l))
    }
}

fn parse_packet<'a>() -> impl Parser<'a, Packet> {
    move |ctx| {
        find_any((parse_list_packet(),parse_val_packet())).parse(ctx)
    }
}

impl FromStr for Packet{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //Not enough documentation on the returned error to do actual error handling here.
        let (_,res) = parse_packet().parse(s).unwrap();
        Ok(res)

    }
}