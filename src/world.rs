use crate::frame::Frame;
use crate::vectors::Vector;
use crate::rays::Ray;
use crate::objects::Sphere;

pub struct World { //Contains all the objects in our "world"
    frame: frame,
    objects: Vector<Sphere>
}

impl World {
    pub fn trace(&self, ray: &Ray){
        //Ray = origin + t*direction, where t is variable
        let object: Sphere = ;
        let max: f64 = -1.0;
        for object in self.objects {
            if object.intersectable(ray) {
                objects.push(object)
            }
        }
    }
}
