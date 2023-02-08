extern crate gl33;
use glm::{*};
pub struct Material{
    ambient : Vector4<f32>,
    diffuse : Vector4<f32>,
    specular : Vector4<f32>,
    shininess : f32,

}
pub struct Mesh{
    vertex_buffer: Vec<f32>,
    tex_buffer : Vec<f32>,
    normal_buffer : Vec<f32>,
    index_buffer : Vec<u32>,
}
pub struct Object{
    mesh : Mesh,
    material : Material,
    texture : &'static str,
    velocity : Vector3<f32>,
    rotational_velocity : Vector3<f32>
}
