use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::error::{Error, parse_posnum, parse_prefix};

type Coordinate = (i32, i32);

pub struct Sensor{
    location: Coordinate,
    beacon_location: Coordinate
}

impl Sensor {
    pub fn get_location(&self)->Coordinate{
        self.location
    }

    pub fn get_beacon(&self)->Coordinate{
        self.beacon_location
    }

    pub fn get_distance(&self)->usize{
        manhatten_dist(self.location,self.beacon_location)
    }

    pub fn cannot_contain_beacon(&self, pos:Coordinate) -> bool {
        self.get_distance()>=manhatten_dist(self.get_location(),pos) &&
            (pos.0!=self.beacon_location.0 || pos.1!=self.beacon_location.1)
    }

    pub fn can_contain_beacon(&self, pos:Coordinate) -> bool {
        self.get_distance()<manhatten_dist(self.get_location(),pos)
    }

    // pub fn debug_can_contain_beacon(&self, pos:Coordinate) -> bool {
    //     println!("distance{}, other_distance{}",self.get_distance(),manhatten_dist(self.get_location(),pos));
    //     println!("extra check:{}",(pos.0!=self.beacon_location.0 || pos.1!=self.beacon_location.1));
    //     self.get_distance()>=manhatten_dist(self.get_location(),pos) &&
    //         (pos.0!=self.beacon_location.0 || pos.1!=self.beacon_location.1)
    // }
}

impl Display for Sensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Sensor at {},{}. closest beacon at {},{}",self.location.0,self.location.1,self.beacon_location.0,self.beacon_location.1)
    }
}

impl FromStr for Sensor{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       // "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
        let (rem,x) =  parse_posnum(parse_prefix(s,"Sensor at x=")?)?;
        let (rem2,y) = parse_posnum(parse_prefix(rem,", y=")?)?;
        let (rem3,x_beacon) = parse_posnum(parse_prefix(rem2,": closest beacon is at x=")?)?;
        let (_,y_beacon) = parse_posnum(parse_prefix(rem3,", y=")?)?;
        Ok(Sensor{ location: (x, y), beacon_location: (x_beacon, y_beacon) })
    }
}

pub fn manhatten_dist(start:Coordinate,end:Coordinate)->usize{
    ((start.0 - end.0).abs() + (start.1 - end.1).abs()) as usize
}