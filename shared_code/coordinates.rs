pub struct Coordinate<T>{
    pub x:T,
    pub y:T
}

impl  <T> Coordinate<T> {
    pub fn new(x:T,y:T)->Coordinate<T>{
        Coordinate{x,y}
    }

    pub fn as_tuple(&self)->(&T,&T){
        (&self.x,&self.y)
    }
}