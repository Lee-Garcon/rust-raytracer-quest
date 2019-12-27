use std::convert::From;

use super::vector::Vector;

pub struct Ray { //Ray object: origin -> origin point, direction -> vector
    //              in direction of ray (unit vector)
    pub origin: Vector,
    pub direction: Vector
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        let unit_direction = direction.unit();
        Ray {origin: origin, direction: unit_direction}
    }
    pub fn values(&self) -> (Vector, Vector){
        (self.origin, self.direction)
    }
    pub fn display(&self) -> String {
        format!("Ray({} -> {})", self.origin.display(), self.direction.display())
    }
}

impl From<(Vector, Vector)> for Ray{
    fn from(t: (Vector, Vector)) -> Self {
        let (origin, direction) = t;
        Ray::new(origin, direction)
    }
}
