use crate::frame::objects::assets::primitives::vector::Vector;
use crate::frame::objects::assets::primitives::ray::Ray;
use crate::frame::objects::assets::intersect::Intersect;
use crate::frame::objects::assets::primitives::color::Color;

pub struct Plane {
    pub p0: Vector,
    pub normal: Vector,
    pub color: Color
}

impl Intersect for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.p0.sub(&ray.origin);
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
    fn surface_normal(&self, hit_point: &Vector) -> Vector {
        self.normal
    }
    fn albedo(&self) -> f32 {
        1 as f32
    }
}
