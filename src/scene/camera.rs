use glm::*;

use crate::types::*;

use super::view_object::ViewObject;

pub struct Camera {
    pub view_obj : ViewObject,
    pub projection : Matrix4<f32>
}

impl Camera {
    pub fn look_at(&self) -> Mat4 {
        return Mat4::look_at_rh(&glm::OPoint::from(self.view_obj.position), &glm::OPoint::from(self.view_obj.position+self.view_obj.direction), &Vec3::new(0.0, 1.0, 0.0))
    }
}