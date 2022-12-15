
type Coordinate = (i32,i32);
///An iterator to iterate over all points exactly distance away from a center
pub struct DiagIterator{
    center:Coordinate,
    distance:usize,
    i:usize
}

impl DiagIterator{
    pub fn new(center:Coordinate,distance:usize)->Self{
        DiagIterator{
            center,
            distance,
            i: 0,
        }
    }

    fn get_max(&self)->usize{
        4*self.distance
    }
}

impl Iterator for DiagIterator{
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let max  = self.get_max() as i32;
        let mut i = self.i as i32;
        let x = self.center.0;
        let y = self.center.1;
        let dist = self.distance as i32;
        let val = if i >= max{
            None
        } else if i < dist{
            Some((x+dist-i,y+i))
        } else if i < dist*2{
            let i = i-dist;
            Some((x-i,y+dist-i))
        } else if i < dist*3{
            let i = i-dist*2;
            Some((x-dist+i,y-i))
        } else {
            let i = i-dist*3;
            Some((x+i,y-dist+i))
        };
        self.i+=1;
        val
    }
}