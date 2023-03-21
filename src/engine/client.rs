use gl33::*;
use glm::{*};
use crate::rendering::draw::draw_object;
use crate::scene::*;
use crate::rendering::*;
use crate::ServerEngine;
use crate::types::*;
use sdl2::image::*;

pub struct Client{
    pub camera : Camera,
    pub server : ServerEngine,
    pub gl : GlFns
}
pub fn rot_align(vec1 : Vec3, vec2 : Vec3) -> Matrix3<f32> {
    let axis = vec1.cross(&vec2);

    let cos_a = vec1.dot(&vec2);
    let k = 1.0 / (1.0 + cos_a);

    return Matrix3::new( (axis.x * axis.x * k) + cos_a,
                 (axis.y * axis.x * k) - axis.z, 
                 (axis.z * axis.x * k) + axis.y,
                 (axis.x * axis.y * k) + axis.z,  
                 (axis.y * axis.y * k) + cos_a,      
                 (axis.z * axis.y * k) - axis.x,
                 (axis.x * axis.z * k) - axis.y,
                 (axis.y * axis.z * k) + axis.x,  
                 (axis.z * axis.z * k) + cos_a,
                 )

}

impl Client {
    pub unsafe fn init_gl(&self) {
        let gl = &self.gl;
        gl.Enable(GL_DEPTH_TEST);
        gl.DepthFunc(GL_LEQUAL);


        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_MIRRORED_REPEAT.0.try_into().unwrap());
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_MIRRORED_REPEAT.0.try_into().unwrap());
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR.0.try_into().unwrap());
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0.try_into().unwrap());
        gl.ClearColor(0.0, 0.0, 0.0, 1.0);
    }
    pub fn update_camera(&mut self, step : f32) {
        self.camera.view_obj.update_cam(step);
    }

    pub unsafe fn draw_scene(&self, frag_shader : &String, vert_shader : &String) {
        for object in &self.server.objects{
            draw_object(self, object, vert_shader, frag_shader)
        }
    }
}