use glm::{*};

pub struct ViewObject{
    position : Vector3<f32>,
    direction : Vector3<f32>,
    projection : Matrix4<f32>
}

impl ViewObject{
    pub fn new(pos : Vector3<f32>, dir : Vector3<f32>, proj : Matrix4<f32>) -> Self{
        return Self{
            position : pos,
            direction : dir,
            projection : proj
        }
    }
}