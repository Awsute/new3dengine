use glm::*;
use gl33::*;
use std::fs;
use crate::rendering::{*, draw::*, uniform::*};
use crate::{camera::*, object::*, engine::*};
pub const LIGHT_RESOLUTION : (i32, i32) = (512, 512);
pub struct Light{
    pub camera : Camera,
    pub color : Vector4<f32>,
    pub strength : f32,
    pub depth_map : u32,
}

impl Light {
}