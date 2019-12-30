use image;
use image::{DynamicImage, GenericImage, Pixel, Rgba};

use crate::frame::objects::element::Element;
use crate::frame::objects::assets::primitives::vector::Vector;
use crate::frame::objects::assets::primitives::ray::Ray;
use crate::frame::objects::intersection::Intersection;
use crate::frame::objects::assets::light::Light;
use crate::frame::objects::assets::primitives::color::Color;
use crate::frame::objects::assets::intersect::Intersect;

pub struct Frame {
    pub dimentions: (u32, u32),
    //pub position: Vector,
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub light: Vec<Light>,
    pub background: Color,
    pub smudge: f64
}

//Based on https://bheisler.github.io/post/writing-raytracer-in-rust-part-2/
impl Frame {
    pub fn create_ray(&self, x: u32, y: u32) -> Ray {
        let (width, height) = self.dimentions;
        assert!(width > height);
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (width as f64) / (height as f64);
        let sensor_x = ((((x as f64 + 0.5) / width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / height as f64) * 2.0) * fov_adjustment;
        Ray {
            origin: Vector{ x: 0.0, y: 0.0, z: 0.0 },
            direction: Vector {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .unit()
        }
    }
    //https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/
    pub fn render(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let mut total: u32 = 0;
        let mut background: u32 = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = self.create_ray(x, y);
                let intersection = self.trace(&ray);
                total += 1;
                let color = match intersection {
                    Some(inter) => self.get_color(&ray, &inter),
                    None => {background += 1; self.background}
                };
                image.put_pixel(x, y, color.to_rgba());
            }
        }
        println!("background: {}", background);
        println!("total: {}", total);
        image
    }
    //https://bheisler.github.io/post/writing-raytracer-in-rust-part-2/
    pub fn get_color(&self, ray: &Ray, intersection: &Intersection) -> Color {
        let hit_point = ray.origin.add(&ray.direction.mul(intersection.distance));
        let surface_normal = intersection.element.surface_normal(&hit_point);
        let mut color = Color::new(0.0 as f32, 0.0 as f32, 0.0 as f32);
        for light in self.light.iter() {
            /*
            let c = {
                let direction_to_light = light.direction.unit().mul(-1 as f64);
                let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light.intensity.min(1.0);
                let light_reflected = intersection.element.albedo() / std::f32::consts::PI;
                let color = intersection.element.color().mul_c(&light.color).mul_s(light_power).mul_s(light_reflected);
                //color.clamp()
                color.modulo(1.0)
            };
            */

            let direction_to_light = light.direction.unit();//.mul(-1 as f64);
            //https://bheisler.github.io/post/writing-raytracer-in-rust-part-2/


            let shadow_ray = Ray {
                origin: hit_point.add(&surface_normal.mul(self.smudge)),
                direction: direction_to_light,
            };
            let in_light = self.trace(&shadow_ray).is_none();
            let light_intensity = if in_light { light.intensity } else { 0.0 };


            //let light_intensity = light.intensity;

            let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
            let light_reflected = intersection.element.albedo() / std::f32::consts::PI;
            let light_color = intersection.element.color().mul_c(&light.color).mul_s(light_power).mul_s(light_reflected);
            color = color + light_color;
        }
        color.clamp()
    }
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        let i1 = self.elements
            .iter()
            .filter_map(|s| match s.intersect(ray) {
                None => None,
                Some(t) => Some(s)
            });
        i1.map(|s| Intersection::new(s.intersect(ray).unwrap(), s))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
        /*
        self.elements
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
            */
    }
}
