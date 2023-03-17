use glm::*;
use gl33::*;
use crate::rendering::*;
use super::camera::*;

pub struct Light{
    pub camera : Camera,
    pub color : Vector4<f32>,
    pub strength : f32
}

impl Light {
}