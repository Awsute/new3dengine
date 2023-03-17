use std::fs::DirEntry;

use glm::*;
use gl33::*;
use crate::types::*;
use crate::rendering::uniform::*;

use super::view_object::ViewObject;

pub struct Camera {
    pub view_obj : ViewObject,
    pub projection : Matrix4<f32>
}

impl Camera {
    pub fn new(position : Vec3, direction : Vec3, projection : Mat4) -> Self{
        return Self{
            view_obj : ViewObject::new(position, direction, Vec3::new(0.0,1.0,0.0), Vec3::zeros(), Vec3::zeros()),
            projection
        }
    }
}