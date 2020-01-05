mod frame;

use self::frame::objects::assets::primitives::vector::Vector;
use self::frame::objects::assets::primitives::color::Color;
use self::frame::frame::Frame;
use self::frame::objects::element::Element;
use self::frame::objects::assets::light::DirectionalLight;
use self::frame::objects::assets::light::SphericalLight;
use self::frame::objects::assets::light::Light;
use self::frame::objects::assets::plane::Plane;
use self::frame::objects::assets::sphere::Sphere;

use image;
use image::{DynamicImage, GenericImage, Pixel, Rgba};

fn main() {
    let sphere1 = Sphere {
        center: Vector::new(0.0, 0.0, -5.0),
        radius: 1.0,
        color: Color::from_tuple_rgb((255, 63, 127))
    };
    let sphere2 = Sphere {
        center: Vector::new(-3.0, 1.0, -6.0),
        radius: 2.0,
        color: Color::from_tuple_rgb((0, 255, 63))
    };
    let sphere3 = Sphere {
        center: Vector::new(2.0, 1.0, -4.0),
        radius: 1.5,
        color: Color::from_tuple_rgb((255, 63, 255))
    };
    let plane1 = Plane {
        p0: Vector::new(0.0, -2.0, -5.0),
        normal: Vector::new(0.0, -1.0, 0.0),
        color: Color::from_tuple_rgb((255, 255, 255))
    };
    let plane2 = Plane {
        p0: Vector::new(0.0, 0.0, -20.0),
        normal: Vector::new(0.0, 0.0, -1.0),
        color: Color::from_tuple_rgb((0, 255, 255))
    };

    let light1 = Light::DirectionalLight(DirectionalLight {
        direction: Vector::new(-2.0, -1.0, 1.0),
        color: Color::from_tuple_rgb((255, 255, 255)),
        intensity: 0.7
    });

    let light2 = Light::DirectionalLight(DirectionalLight {
        direction: Vector::new(2.0, -1.0, 1.0),
        color: Color::from_tuple_rgb((255, 255, 255)),
        intensity: 0.7
    });
    let light3 = Light::SphericalLight(SphericalLight {
        center: Vector::new(-0.5, -1.0, -2.0),
        color: Color::from_tuple_rgb((255, 255, 255)),
        intensity: 5.0
    });
    /*
    let light3 = Light {
        direction: Vector::new(0.0, -1.0, 1.0),
        color: Color::from_tuple_rgb((255, 255, 255)),
        intensity: 0.5
    };
    */
    let mut element_vector = vec![
        Element::Sphere(sphere1),
        Element::Sphere(sphere2),
        Element::Sphere(sphere3),
        //Element::Plane(plane2)
    ];
    let mut light_vector = vec![
        light1,
        //light2,
        light3
    ];

    let background = Color::from_tuple_rgb((63, 63, 63));

    let mut frame = Frame {
        dimentions: (1920, 1080),
        width: 1920,
        height: 1080,
        fov: 90.0,
        elements: element_vector,
        light: light_vector,
        background: background,
        smudge: 1e-12,
	max_recursion_depth: 10
    };

    let image = frame.render();
    image.save("trace.png");

    /*
    let ray = frame.create_ray(2, 3); //ray that we know should be a background
    let intersection = frame.trace(&ray);
    //intersection is None
    /*
    match intersection {
        Some(t) => println!("Bruh, this thing has something in it"), //WHY WTF IS WRONG WITH THIS THING
        None => println!("Nah, this thing doesn't have anything in it")
    };
    */
    match intersection {
        Some(t) => {
            println!("Distance: {}, Hit point: {}", t.distance, ray.origin.add(&ray.direction.mul(t.distance)).debug());
            t.element.debug();
        }
        None => println!("Nah, this thing doesn't have anything in it")
    };
    */
}
/*
fn main() {
    let c1 = Color::new(1.0, 1.0, 1.0);
    let c2 = Color::new(0.0, 1.0, 0.5);
    let c3 = c1.mul_c(&c2);
    println!("{} {} {}", c3.red, c3.green, c3.blue);
}
*/
