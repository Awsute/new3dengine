use glm::*;

use super::view_object::ViewObject;

pub struct Light{
    view_object : ViewObject,
    color : Vector4<f32>,
    strength : f32
}


impl Light{
    pub fn new(position : Vector3<f32>, direction : Vector3<f32>, proj_matrix : Matrix4<f32>, color : Vector4<f32>, strength : f32) -> Self{
        return Self { 
            view_object: ViewObject::new(position, direction, proj_matrix), 
            color: color, 
            strength: strength
        }
    }
}