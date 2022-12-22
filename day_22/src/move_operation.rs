use crate::facing::{ Rotation};

pub struct MoveOperation {
    pub rotation:Rotation,
    pub steps:usize
}

impl MoveOperation{
    pub fn new(rotation:Rotation,steps:usize)->MoveOperation
    {
        MoveOperation{ rotation, steps }
    }
}