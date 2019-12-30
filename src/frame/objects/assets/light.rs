use crate::frame::objects::assets::primitives::vector::Vector;
use crate::frame::objects::assets::primitives::color::Color;


pub struct DirectionalLight {
    pub direction: Vector,
    pub color: Color,
    pub intensity: f32
}

pub struct SphericalLight {
    pub center: Vector,
    pub color: Color,
    pub intensity: f32
}

pub enum Light {
    DirectionalLight(DirectionalLight),
    SphericalLight(SphericalLight)
}
