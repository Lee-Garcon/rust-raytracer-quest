use crate::frame::objects::assets::primitives::vector::Vector;
use crate::frame::objects::assets::primitives::ray::Ray;

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, hit_point: &Vector) -> Vector;
    fn albedo(&self) -> f32;
}
