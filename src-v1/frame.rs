extern crate img;
use crate::vectors::Vector;
use crate::rays::Ray;
use crate::objects;
use std::mem;

//Saving for a Better Day (when I can make a more complex frame)

/*
pub struct Frame {
    dimentions: (u32, u32),
    direction: Vector,
    position: Vector,
    rays: Vec<Ray>
}

impl Frame {
    pub fn new(dimentions: (u32, u32), direction: Vector, position: Vector) -> Frame {
        Frame { dimentions: dimentions, direction: direction, position: position,
            rays: Frame::generate(dimentions, direction, position, pixel_width=0.5)
        }
    }
    fn generate(dimentions: (u32, u32), direction: Vector, position: Vector, pixel_width) -> Vec<Ray> { //generate a collection of rays
        let (x, y) = dimentions;
        for y_val in 0..y { //for every row...
            for x_val in 0..x {


            } //end x for
        } //end y for
    }
}
*/

pub struct Frame { // Contains all the rays that we trace (at least initially). Used as a camera of sorts.
    dimention: (u32, u32),
    fov: f64,
    direction: Vector,
    position: Vector,
    //direction: Vector, //most likely (1, 0, 0)
    //position: Vector,
    //rays: Vec<Ray>,
    elements: Vec<objects::Element>
}

//Based on https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/

impl Frame {

    pub fn create_ray(x: u32, y: u32, frame: &Frame) -> Ray {
        let (width, height) = frame.dimention;
        assert!(width > height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
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

    pub fn render(&self) -> DynamicImage {
        let (width, height) = self.dimentions;
        for x in 0..width {
            for y in 0..height {
                let ray = Frame::create_ray(x, y, &self);
                self.trace(&ray);
            }
        }
    }
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
    /*
    fn generate(dimention: (u32, u32)) -> Vec<Ray> { //generate a collection of rayso
        //direction is implied to be (1.0, 0.0, 0.0)
        let direction = Vector::new(0.0, 0.0, 1.0);
        let mut v: Vec<Ray> = vec![];
        let (x, y) = dimentions;
        for y_val in 0..y { //for every row...
            for x_val in 0..x {
                let (pos_x, pos_y, pos_z) = position.values();
                let pos = Vector::from(
                    (
                        pos_x + pixel_size * ((x_val as f64 - (x as f64)/2 as f64) as f64 + 0.5),
                        pos_y + pixel_size * ((y_val as f64 - (y as f64)/2 as f64) as f64 + 0.5),
                        pos_z
                    )
                );
                v.push(Ray::from((pos, direction)));
            } //end x for
        } //end y for
        v
    }
    */
}
