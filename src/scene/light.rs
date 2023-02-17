use glm::*;

use super::view_object::ViewObject;

pub struct Light{
    pub view_object : ViewObject,
    pub color : Vector4<f32>,
    pub strength : f32
}