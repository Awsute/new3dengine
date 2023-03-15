use glm::{*};
use crate::types::*;
pub struct ViewObject{
    pub position : Vector3<f32>,
    pub direction : Vector3<f32>,
    pub velocity : Vector3<f32>,
    pub rotational_velocity : Vector3<f32>,
}


impl ViewObject{
    pub fn empty() -> Self{
        Self { 
            position: Vec3::new(0.0,0.0,0.0), 
            direction: Vec3::new(0.0,0.0,1.0), 
            velocity: Vec3::new(0.0,0.0,0.0), 
            rotational_velocity: Vec3::new(0.0,0.0,0.0)
        }
    }
}