use std::convert::From;
use std::mem;

//Each Vector is 24 bytes.

#[derive(Copy, Clone)]
pub struct Vector { // Stores 3 vals in 3 dimentions. Used as a vector or a point.
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector{ x: x, y: y, z: z }
    }
    pub fn magnitude(&self) -> f64{
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt() as f64
    }
    pub fn unit(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector::div(&self, magnitude)
    }
    pub fn add(v1: &Vector, v2: &Vector) -> Vector {
        Vector::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z)
    }
    pub fn sub(v1: &Vector, v2: &Vector) -> Vector {
        Vector::new(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z)
    }
    pub fn mul(v1: &Vector, a: f64) -> Vector {
        Vector::new(v1.x * a, v1.y * a, v1.z * a)
    }
    pub fn div(v1: &Vector, a: f64) -> Vector {
        Vector::new(v1.x / a, v1.y / a, v1.z / a)
    }
    pub fn dot_product(v1: &Vector, v2: &Vector) -> f64 {
        (v1.x * v2.x + v1.y * v2.y + v1.z * v2.z)
    }
    pub fn cross_product(v1: &Vector, v2: &Vector) -> Vector {
        Vector::new(v1.y*v2.z - v1.z*v2.y, v1.z*v2.x - v1.x*v2.z, v1.x*v2.y - v1.y*v2.z)
    }
    pub fn display(&self) -> String {
        format!("V({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
    pub fn debug(&self) -> String {
        format!("Vector({}, {}, {})[{}]", self.x, self.y, self.z, self.magnitude())
    }
    pub fn bytes() -> i32 {
        let bytes = mem::size_of::<Vector>();
        bytes as i32
    }
    pub fn values(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

impl From<(f64, f64, f64)> for Vector {
    fn from(t: (f64, f64, f64)) -> Self {
        let (x, y, z) = t;
        Vector {x: x, y: y, z: z}
    }
}
