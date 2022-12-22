use std::collections::HashMap;
use crate::facing::{Facing, Rotation};

pub type Zone = (usize, usize);
pub type ZoneMap = HashMap<(Zone,Facing),(Zone,Rotation)>;
pub const DO_P2:bool = true;
pub const DO_DEBUG:bool = false;
pub const INPUT_FILE:&'static str = if DO_DEBUG {"testinput.txt"} else {"input.txt"};
pub const ZONE_SIZE: usize = if DO_DEBUG {4} else { 50 };