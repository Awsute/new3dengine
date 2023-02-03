use glm::*;

use super::view_object::ViewObject;

pub struct Light{
    view_object : ViewObject,
    color : Vec4,
    strength : f32
}


impl Light{
    pub fn new(position : Vec3, direction : Vec3, proj_matrix : Mat4, color : Vec4, strength : f32) -> Self{
        return Self { 
            view_object: ViewObject::new(position, direction, proj_matrix), 
            color: color, 
            strength: strength
        }
    }
}