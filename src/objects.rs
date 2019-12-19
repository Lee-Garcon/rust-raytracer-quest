use crate::rays::Ray;
use crate::vectors::Vector;

trait Intersectable {
    pub fn intersect(&self, ray: &Ray) -> (bool, f64);
}

pub enum MaterialType {
    Matte,
    Reflective(f64)
}

struct Sphere { //Sphere
    //material_type: MaterialType,
    center: Vector,
    radius: f64,
    color: (f32, f32, f32)
}

impl Intersectable for Sphere {
    pub fn intersect(&self, ray: &Ray) -> (bool, f64) {
        let l: Vector = Vector::sub(&self.center, &ray.origin);
        let adj = Vector::dot_product(&l, &ray.direction);
        let d2 = Vector::dot_product(&l, &l) - (adj * adj);
        (d2 < (self.radius * self.radius), l.magnitude())
    }
}
