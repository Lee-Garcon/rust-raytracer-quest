use crate::frame::objects::assets::primitives::vector::Vector;
use crate::frame::objects::assets::primitives::color::Color;

pub struct Light {
    pub direction: Vector,
    pub color: Color,
    pub intensity: f32
}
