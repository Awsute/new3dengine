use gl33::GlFns;
use glm::{*};
use crate::types::*;
use crate::rendering::uniform::*;
pub struct ViewObject{
    pub position : Vector3<f32>,
    pub forward : Vector3<f32>,
    pub up : Vector3<f32>,
    pub velocity : Vector3<f32>,
    pub rotational_velocity : Vector3<f32>,
}


impl ViewObject {
    pub fn new(position : Vec3, forward : Vec3, up : Vec3, velocity : Vec3, rotational_velocity : Vec3) -> Self {
        return Self { position, forward, up, velocity, rotational_velocity}
    }
    pub fn empty() -> Self {
        Self { 
            position: Vec3::new(0.0,0.0,0.0), 
            forward: Vec3::new(0.0,0.0,1.0), 
            up : Vec3::new(0.0,1.0,0.0),
            velocity: Vec3::new(0.0,0.0,0.0), 
            rotational_velocity: Vec3::new(0.0,0.0,0.0)
        }
    }
    pub fn from_position(position : Vec3) -> Self {
        Self::new(position, Vec3::new(0.0,0.0,1.0), Vec3::new(0.0,1.0,0.0), Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0))
    }
    pub fn look_at(&self) -> Mat4 {
        return Mat4::look_at_rh(&glm::OPoint::from(self.position), &glm::OPoint::from(self.position+self.forward), &Vec3::new(0.0,1.0,0.0))
    }
    pub fn look_at_up(&self, up : Vec3) -> Mat4{
        return Mat4::look_at_rh(&glm::OPoint::from(self.position), &glm::OPoint::from(self.position+self.forward), &up)
    }
    pub fn update_cam(&mut self, step : f32) {

        let point_at = self.look_at().try_inverse().unwrap();
        let adjusted_rot = Mat4::new_rotation((point_at*self.rotational_velocity.insert_row(3, 0.0)).remove_row(3)*step);
        let fwd_rot = adjusted_rot * self.forward.insert_row(3, 0.0);
        let up_rot = adjusted_rot * self.up.insert_row(3, 0.0);
        self.forward = fwd_rot.remove_row(3);
        self.up = up_rot.remove_row(3);
        
        let transformed_velocity = point_at*self.velocity.insert_row(3,0.0);
        self.position = self.position - transformed_velocity.remove_row(3)*step;
    }
    pub fn update_object(&mut self, step : f32) {

        let point_at = self.look_at_up(self.up).try_inverse().unwrap();
        let adjusted_rot = Mat4::new_rotation((point_at*self.rotational_velocity.insert_row(3, 0.0)).xyz()*step);
        let fwd_rot = adjusted_rot * self.forward.insert_row(3, 0.0);
        let up_rot = adjusted_rot * self.up.insert_row(3, 0.0);
        self.forward = fwd_rot.xyz();
        self.up = up_rot.xyz();
        
        self.position = self.position + self.velocity*step;
    }
}