use std::ops::Mul;
use std::ops::Add;
use image;
use image::{DynamicImage, GenericImage, Pixel, Rgba};

#[derive(Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

const GAMMA: f32 = 2.2;
fn gamma_encode(a: f32) -> f32 {
    a.powf(1.0 / GAMMA)
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color{red: r, green: g, blue: b}
    }
    pub fn from_tuple(t: (f32, f32, f32)) -> Color {
        Color{red: t.0, green: t.1, blue: t.2}
    }
    pub fn from_tuple_rgb(t: (u32, u32, u32)) -> Color {
        Color{
            red: (t.0 / 255) as f32,
            green: (t.1 / 255) as f32,
            blue: (t.2 / 255) as f32
        }
    }
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(
            (gamma_encode(self.red) * 255.0) as u8,
            (gamma_encode(self.green) * 255.0) as u8,
            (gamma_encode(self.blue) * 255.0) as u8,
            255
        )
    }
    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0)
        }
    }
    pub fn modulo(&self, d: f32) -> Color {
        Color {
            red: self.red % d,
            green: self.green % d,
            blue: self.blue % d
        }
    }
    pub fn mul_c(&self, c: &Color) -> Color {
        Color {
            red: self.red * c.red,
            green: self.green * c.green,
            blue: self.blue * c.blue
        }
    }
    pub fn mul_s(&self, s: f32) -> Color {
        Color {
            red: self.red * s,
            green: self.green * s,
            blue: self.blue * s
        }
    }
    pub fn values(&self) -> (f32, f32, f32) {
        (self.red, self.green, self.blue)
    }

    //Debug purposes
    pub fn debug(&self) {
        let (red, green, blue) = self.values();
        println!("{}, {}, {}", red*(255 as f32), green*(255 as f32), blue*(255 as f32));
    }
}
//https://github.com/bheisler/raytracer/blob/master/src/scene.rs
impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}
