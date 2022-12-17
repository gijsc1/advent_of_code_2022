use crate::tower::Tower;

pub type Xcoord = isize;
pub type Ycoord = isize;
pub type Coordinate = (Xcoord,Ycoord);
pub type CacheKey = (usize,usize,Tower);
pub const TOWERWIDTH:usize= 7;
pub const LEFT_SPAWN_DISTANCE:usize = 2;
pub const UP_SPAWN_DISTANCE:usize = 3;
pub const TOWER_HASH_LIMIT:usize = 20;