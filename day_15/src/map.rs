use crate::sensor::Sensor;
use crate::layout::Layout;
use crate::map::Field::MaybeBeacon;
use crate::layout::num_traits::Zero;

#[derive(Clone)]
pub enum Field{
    NoBeacon,
    MaybeBeacon
}

impl Zero for Field{
    fn zero() -> Self {
        MaybeBeacon
    }
}

pub struct Map{
    layout:Layout<Field>,
    sensors:Vec<Sensor>
}

impl Map {
//     fn test(&mut self){
//         let x = self.layout.get_val(0,0);
//     }
}