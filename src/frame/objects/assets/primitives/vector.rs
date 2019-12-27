use std::convert::From;

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

    //Discontinued functions and methods
    /*
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
    pub fn dot(&self, v: &Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    pub fn dot_product(v1: &Vector, v2: &Vector) -> f64 {
        (v1.x * v2.x + v1.y * v2.y + v1.z * v2.z)
    }
    pub fn cross(&self, v: &Vector) -> Vector {
        Vector::new(self.y*v.z - self.z*v.y, self.z*v.x - self.x*v.z, self.x*v.y - self.y*v.z)
    }
    pub fn cross_product(v1: &Vector, v2: &Vector) -> Vector {
        Vector::new(v1.y*v2.z - v1.z*v2.y, v1.z*v2.x - v1.x*v2.z, v1.x*v2.y - v1.y*v2.z)
    }
    */

    //Vector operations (standard)
    pub fn add(&self, v: &Vector) -> Vector {
        Vector::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
    pub fn sub(&self, v: &Vector) -> Vector {
        Vector::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }

    //Vector scalar operations
    pub fn mul(&self, a: f64) -> Vector {
        Vector::new(self.x * a, self.y * a, self.z * a)
    }
    pub fn div(&self, a: f64) -> Vector {
        Vector::new(self.x / a, self.y / a, self.z / a)
    }

    //Special multiplication
    pub fn dot(&self, v: &Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    pub fn cross(&self, v: &Vector) -> Vector {
        Vector::new(self.y*v.z - self.z*v.y, self.z*v.x - self.x*v.z, self.x*v.y - self.y*v.z)
    }

    //Other functions
    pub fn display(&self) -> String {
        format!("V({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
    pub fn debug(&self) -> String {
        format!("Vector({}, {}, {})[{}]", self.x, self.y, self.z, self.magnitude())
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
