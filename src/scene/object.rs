extern crate gl33;
use glm::*;

pub struct Mesh{
    vertex_buffer: Vec<f32>,
    tex_buffer : Vec<f32>,
    normal_buffer : Vec<f32>,
    index_buffer : Vec<u32>,
}


impl Mesh{
    pub fn from_obj_file(path : &str){

    }
}