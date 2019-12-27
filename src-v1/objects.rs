use crate::rays::Ray;
use crate::vectors::Vector;

//Based on https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/

trait Intersectable {
    pub fn intersect(&self, ray: &Ray) -> Option<f64>;
}

pub struct Sphere { //Sphere
    //material_type: MaterialType,
    pub center: Vector,
    pub radius: f64,
    pub color: (f32, f32, f32)
}

pub struct Plane {
    pub p0: Vector,
    pub normal: Vector,
    pub color: (f32, f32, f32)
}

pub struct Material {
    pub color: (f32, f32, f32),
    pub material: MaterialType
}

pub enum MaterialType {
    Matte,
    Reflective(f64)
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane)
}

impl Element {
    pub fn color(&self) -> (f32, f32, f32) {
        match *self {
            Element::Sphere(ref s) => s.color,
            Element::Plane(ref p) => p.color
        }
    }
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray)
        }
    }
}

// https://bheisler.github.io/post/writing-raytracer-in-rust-part-2/
impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = Vector::sub(&self.p0, &ray.origin);
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Sphere
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, object: &'b Sphere) -> Intersection<'b> {
        Intersection{ distance: distance, object: object }
    }
}

impl Intersectable for Sphere {
    pub fn intersect(&self, ray: &Ray) -> Option<f64> {
       let l: Vector = Vector::sub(&self.center, &ray.origin);
       let adj = l.dot(&ray.direction);
       let d2 = l.dot(&l) - (adj * adj);
       let radius2 = self.radius * self.radius;
       if d2 > radius2 {
           return None;
       }
       let thc = (radius2 - d2).sqrt();
       let t0 = adj - thc;
       let t1 = adj + thc;

       if t0 < 0.0 && t1 < 0.0 {
           return None;
       }

       let distance = if t0 < t1 { t0 } else { t1 };
       Some(distance)
    }
}
