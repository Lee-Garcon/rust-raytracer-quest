use crate::vectors::Vector;
use crate::rays::Ray;
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
    dimentions: (u32, u32),
    direction: Vector, //most likely (1, 0, 0)
    position: Vector,
    rays: Vec<Ray>
}

impl Frame {
    pub fn new(dimentions: (u32, u32), position: Vector) -> Frame {
        Frame { dimentions: dimentions, direction: Vector::new(0.0, 0.0, 1.0), position: position,
            rays: Frame::generate(dimentions, position, 0.5)
        }
    }
    fn generate(dimentions: (u32, u32), position: Vector, pixel_size: f64) -> Vec<Ray> { //generate a collection of rays
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
    pub fn ray_num(&self) -> u32 {
        self.rays.len() as u32
    }
    pub fn bytes(&self) -> u32 {
        let vec_size: u32 = Vector::bytes() as u32;
        (8 + (2u32 + self.ray_num()) * vec_size) as u32
    }
}
