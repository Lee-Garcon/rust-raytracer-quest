use crate::frame::objects::assets::primitives::vector::Vector;
use crate::frame::objects::assets::primitives::ray::Ray;
use crate::frame::objects::assets::intersect::Intersect;
use crate::frame::objects::assets::primitives::color::Color;

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub color: Color
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l: Vector = self.center.sub(&ray.origin);
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;
        if t0 <= 0.0 && t1 <= 0.0 {
            return None;
        } else if t0 <= 0.0 {
            Some(t1)
        } else if t1 <= 0.0 {
            Some(t0)
        } else {
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }
    }
    fn surface_normal(&self, hit_point: &Vector) -> Vector {
        hit_point.mul(-1 as f64)
    }
    fn albedo(&self) -> f32 {
        1 as f32
    }
}
