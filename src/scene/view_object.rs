use glm::*;

pub struct ViewObject{
    position : Vec3,
    direction : Vec3,
    projection : Mat4
}

impl ViewObject{
    pub fn new(pos : Vec3, dir : Vec3, proj : Mat4) -> Self{
        return Self{
            position : pos,
            direction : dir,
            projection : proj
        }
    }
}