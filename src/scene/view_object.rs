use glm::{*};

pub struct ViewObject{
    pub position : Vector3<f32>,
    pub direction : Vector3<f32>,
    pub projection : Matrix4<f32>,
    pub velocity : Vector3<f32>,
    pub rotational_velocity : Vector3<f32>,
}
