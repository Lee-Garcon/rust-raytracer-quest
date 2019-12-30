//Bundles all objects together into a single enum

use crate::frame::objects::assets::plane::Plane;
use crate::frame::objects::assets::sphere::Sphere;
use crate::frame::objects::assets::intersect::Intersect;
use crate::frame::objects::assets::primitives::ray::Ray;
use crate::frame::objects::assets::primitives::vector::Vector;
use crate::frame::objects::assets::primitives::color::Color;

pub enum Element {
    Plane(Plane),
    Sphere(Sphere)
}

impl Element {
    pub fn color(&self) -> Color {
        match *self {
            Element::Plane(ref p) => p.color,
            Element::Sphere(ref s) => s.color
        }
    }
    pub fn debug(&self){
        match *self {
            Element::Plane(ref p) => println!("I'm a fucking plane at {}", p.p0.display()),
            Element::Sphere(ref s) => println!("I'm a fucking sphere at {}", s.center.display())
        }
    }
}

// https://bheisler.github.io/post/writing-raytracer-in-rust-part-2/
impl Intersect for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Plane(ref p) => p.intersect(ray),
            Element::Sphere(ref s) => s.intersect(ray)
        }
    }
    fn surface_normal(&self, hit_point: &Vector) -> Vector {
        match *self {
            Element::Plane(ref p) => p.surface_normal(hit_point),
            Element::Sphere(ref s) => s.surface_normal(hit_point)
        }
    }
    fn albedo(&self) -> f32 {
        match *self {
            Element::Plane(ref p) => p.albedo(),
            Element::Sphere(ref s) => s.albedo()
        }
    }
}
