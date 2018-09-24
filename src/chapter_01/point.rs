
#[derive(Debug,  PartialEq)]
pub struct Point{ x:i64, y:i64}

/**
Point is a struct for a geometric point
*/
impl Point{
    pub fn new(x:i64, y:i64)->Self{
        /**
        new:  this is how you create a point
         */
        return Point{x, y};
    }

    pub fn distance_to_origin(&self) -> f64{
        /// measures distance to (0,0)
        return ((self.x*self.x + self.y*self.y) as f64).sqrt();
    }
    
    pub fn distance(&self, other:&Point)-> f64{
        return(
            ((self.x - other.x).pow(2) +
             (self.y - other.y).pow(2)) as f64
        ).sqrt();
    }

    pub fn move_x(&mut self, dx:i64){
        self.x += dx;
    }
}

